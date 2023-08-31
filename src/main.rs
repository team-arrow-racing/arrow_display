//! cortex-m-rtic example
//! Tested on BlackPill dev board with stm32l411ceu microcontroller
//! The LCD RESET pin was hard puled to Vcc therefore
//! DummyOutputPin was used as the reset pin

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32l4xx_hal::pac)]
mod app {
    use display_interface_spi::SPIInterface;
    use embedded_hal::spi::{Mode, Phase, Polarity};
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        pixelcolor::Rgb565,
        prelude::*,
        text::{Alignment, Text},
        primitives::{Circle, PrimitiveStyle}
    };

    use ili9341::{DisplaySize240x320, Ili9341, Orientation};
    use stm32l4xx_hal::{
        self,
        delay::{Delay, DelayCM},
        prelude::*,
        spi::Spi,
        gpio::{Alternate, Output, PushPull}, 
    };

    /// SPI mode
    pub const MODE: Mode = Mode {
        phase: Phase::CaptureOnFirstTransition,
        polarity: Polarity::IdleLow,
    };

    #[derive(Default)]

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = cx.device;

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        /*
         *  The ILI9341 driver
         */
        // Configure SPI
        let lcd_clk = gpioa.pa5.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let lcd_miso = gpioa.pa6.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let lcd_mosi = gpioa.pa7.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let lcd_cs = gpioa
            .pa4
            .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);

        let lcd_spi = Spi::spi1(
            dp.SPI1,
            (lcd_clk, lcd_miso, lcd_mosi),
            MODE,
            16.MHz(),
            clocks,
            &mut rcc.apb2,
        );
        
        // let lcd_clk = gpiob.pb0.into_alternate();
        // let lcd_miso = NoMiso {};
        // let lcd_mosi: stm32l4xx_hal::gpio::Pin<'A', 10, stm32l4xx_hal::gpio::Alternate<6>> = gpioa.pa10.into_alternate().internal_pull_up(true);
        let lcd_dc = gpioc.pc3.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
        // let lcd_cs = gpiob.pb2.into_push_pull_output();
        // let mode = Mode {
        //     polarity: Polarity::IdleLow,
        //     phase: Phase::CaptureOnFirstTransition,
        // };
        // let lcd_spi = dp
        //     .SPI5
        //     .spi((lcd_clk, lcd_miso, lcd_mosi), mode, 2.MHz(), &clocks);
        let spi_iface = SPIInterface::new(lcd_spi, lcd_dc, lcd_cs);
        let reset = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        // let mut delay = DelayCM::new(clocks);
        let mut delay: Delay = Delay::new(cx.core.SYST, clocks);
        let mut lcd = Ili9341::new(
            spi_iface,
            reset,
            &mut delay,
            Orientation::PortraitFlipped,
            DisplaySize240x320,
        )
        .unwrap();
        // let data = [128];
        let data = [128, 90, 65, 89, 65];

        // lcd.draw_raw_iter(20, 20, 200, 200, data).unwrap();
        
        lcd.draw_raw_slice(20, 20, 150, 150, data.as_slice()).unwrap();

        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

        
// Create a text at position (20, 30) and draw it using the previously defined style
        // Text::with_alignment(
        //     "First line",
        //     Point::new(20, 30),
        //     style,
        //     Alignment::Center,
        // )
        // .draw(&mut lcd)
        // .unwrap();
        let c = Circle::new(
            Point { x: 50, y: 50 }, 40).into_styled(
            PrimitiveStyle::with_fill(Rgb565::RED)
        );

        // c.draw(&mut lcd);

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