//! cortex-m-rtic example
//! Tested on BlackPill dev board with stm32l411ceu microcontroller
//! The LCD RESET pin was hard puled to Vcc therefore
//! DummyOutputPin was used as the reset pin

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
mod lcd;
#[rtic::app(device = stm32l4xx_hal::pac)]
mod app {
    use stm32l4xx_hal::{
        self,
        delay::{Delay, DelayCM},
        gpio::{Alternate, Output, PushPull},
        prelude::*,
    };

    #[derive(Default)]
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut pwr = cx.device.PWR.constrain(&mut rcc.apb1r1);
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb2);

        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);


        let mut rs = gpioa
            .pa0
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let mut en = gpioa
            .pa1
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let mut d4 = gpiob
            .pb7
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let mut d5 = gpiob
            .pb6
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let mut d6 = gpiob
            .pb1
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let mut d7 = gpioa
            .pa8
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let mut delay = Delay::new(cx.core.SYST, clocks);

        let mut lcd = crate::lcd::LCD::new(
            &mut rs, &mut en, &mut d4, &mut d5, &mut d6, &mut d7, &mut delay,
        );
        lcd.init();
        lcd.send_string("Ligma world");
        
        defmt::debug!("wow");

        (Shared {}, Local {}, init::Monotonics())
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
