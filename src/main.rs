//! cortex-m-rtic example
//! Tested on BlackPill dev board with stm32l411ceu microcontroller
//! The LCD RESET pin was hard puled to Vcc therefore
//! DummyOutputPin was used as the reset pin

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
mod lcd;

pub mod write_to {
    use core::cmp::min;
    use core::fmt;

    pub struct WriteTo<'a> {
        buffer: &'a mut [u8],
        // on write error (i.e. not enough space in buffer) this grows beyond
        // `buffer.len()`.
        used: usize,
    }

    impl<'a> WriteTo<'a> {
        pub fn new(buffer: &'a mut [u8]) -> Self {
            WriteTo { buffer, used: 0 }
        }

        pub fn as_str(self) -> Option<&'a str> {
            if self.used <= self.buffer.len() {
                // only successful concats of str - must be a valid str.
                use core::str::from_utf8_unchecked;
                Some(unsafe { from_utf8_unchecked(&self.buffer[..self.used]) })
            } else {
                None
            }
        }
    }

    impl<'a> fmt::Write for WriteTo<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.used > self.buffer.len() {
                return Err(fmt::Error);
            }
            let remaining_buf = &mut self.buffer[self.used..];
            let raw_s = s.as_bytes();
            let write_num = min(raw_s.len(), remaining_buf.len());
            remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
            self.used += raw_s.len();
            if write_num < raw_s.len() {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    pub fn show<'a>(
        buffer: &'a mut [u8],
        args: fmt::Arguments,
    ) -> Result<&'a str, fmt::Error> {
        let mut w = WriteTo::new(buffer);
        fmt::write(&mut w, args)?;
        w.as_str().ok_or(fmt::Error)
    }
}

#[rtic::app(device = stm32l4xx_hal::pac, dispatchers = [SPI1, SPI2, SPI3, QUADSPI])]
mod app {
    use stm32l4xx_hal::{
        self,
        delay::{Delay, DelayCM},
        gpio::{Alternate, Output, PushPull},
        prelude::*,
    };

    use dwt_systick_monotonic::{fugit, DwtSystick};

    // const DEVICE: device::Device = device::Device::SteeringWheel;
    const SYSCLK: u32 = 80_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = DwtSystick<SYSCLK>;
    pub type Duration = fugit::TimerDuration<u64, SYSCLK>;
    pub type Instant = fugit::TimerInstant<u64, SYSCLK>;

    pub const WARNING_CODES: &str = "ABCDEFG";

    #[derive(Default)]
    #[shared]
    struct Shared {
        speed: u8,
        battery: u8,
        temperature: u8,
        left_indicator: bool,
        right_indicator: bool,
        warnings: [u8; 6]
    }

    #[local]
    struct Local {
        lcd: crate::lcd::LCD
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut pwr = cx.device.PWR.constrain(&mut rcc.apb1r1);
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb2);

        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        // configure monotonic time
        let mono = DwtSystick::new(
            &mut cx.core.DCB,
            cx.core.DWT,
            cx.core.SYST,
            clocks.sysclk().to_Hz(),
        );

        let rs = gpioa
            .pa0
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let en = gpioa
            .pa1
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let d4 = gpiob
            .pb7
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let d5 = gpiob
            .pb6
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let d6 = gpiob
            .pb1
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let d7 = gpioa
            .pa8
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        // let delay = Delay::new(cx.core.SYST, clocks);
        let delay_cm = DelayCM::new(clocks);

        let mut lcd = crate::lcd::LCD::new(
            rs, en, d4, d5, d6, d7, delay_cm,
        );
        lcd.init();
        lcd.set_position(0, 0);
        lcd.send_string("Ligma balls");
        lcd.set_position(0, 1);
        lcd.send_string("Wowzers!");

        let warnings = [0; 6];
        
        defmt::debug!("wow");

        update_display::spawn().unwrap();
        toggle_indicators::spawn().unwrap();
        toggle_warnings::spawn().unwrap();

        (
            Shared {
                speed: 100,
                battery: 12,
                temperature: 65,
                left_indicator: false,
                right_indicator: false,
                warnings: [1, 0, 0, 0, 0, 0]
            }, Local {
                lcd
            },
            init::Monotonics(mono))
    }

    #[task(shared = [speed, battery, temperature, left_indicator, right_indicator, warnings], local=[lcd])]
    fn update_display(cx: update_display::Context) {
        let speed = cx.shared.speed;
        let battery = cx.shared.battery;
        let temp = cx.shared.temperature;
        let l_ind = cx.shared.left_indicator;
        let r_ind = cx.shared.right_indicator;
        let warns = cx.shared.warnings;
       

        let lcd = cx.local.lcd;

        (speed, battery, temp, l_ind, r_ind, warns).lock(|speed, battery, temp, l_ind, r_ind, warns| {
            // Display look should as following
            // |L_100_100_100_R|
            // |    ABCDEFG    |
            // Start by clearing everything
            lcd.clear_display();
            // Set indicator status
            if *l_ind {
                lcd.set_position(0, 0);
                lcd.send_string("L");
            }

            if *r_ind {
                lcd.set_position(15, 0);
                lcd.send_string("R");
            }

            let mut data_buf = [0; 3];

            let mut vehicle_data = crate::write_to::show(
                &mut data_buf,
                format_args!(
                    "{}",
                    battery
                ),
            )
            .unwrap();

            lcd.set_position(2, 0);
            lcd.send_string(vehicle_data);

            vehicle_data = crate::write_to::show(
                &mut data_buf,
                format_args!(
                    "{}",
                    speed
                ),
            )
            .unwrap();

            lcd.set_position(6, 0);
            lcd.send_string(vehicle_data);

            vehicle_data = crate::write_to::show(
                &mut data_buf,
                format_args!(
                    "{}",
                    temp
                ),
            )
            .unwrap();

            lcd.set_position(10, 0);
            lcd.send_string(vehicle_data);
            
            // Warnings
            for i in 0..warns.len() {
                lcd.set_position((4 + i).try_into().unwrap(), 1);
                if warns[i] == 1 {
                    lcd.send_data(WARNING_CODES.chars().nth(i).unwrap().try_into().unwrap());
                } else {
                    lcd.send_string("_");
                }
            }

            update_display::spawn_after(Duration::millis(10)).unwrap();

        });
    }

    #[task(shared = [left_indicator, right_indicator])]
    fn toggle_indicators(cx: toggle_indicators::Context) {
        let l_ind = cx.shared.left_indicator;
        let r_ind = cx.shared.right_indicator;

        (l_ind, r_ind).lock(|l_ind, r_ind| {
            *l_ind = !*l_ind;
            *r_ind = !*r_ind;
        });

        toggle_indicators::spawn_after(Duration::millis(2000)).unwrap();
    }

    #[task(shared = [warnings])]
    fn toggle_warnings(cx: toggle_warnings::Context) {
        let mut warns = cx.shared.warnings;

        warns.lock(|warns| {
            warns[..].rotate_right(1);
        });

        toggle_warnings::spawn_after(Duration::millis(1000)).unwrap();
    }

    #[idle(local = [])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
