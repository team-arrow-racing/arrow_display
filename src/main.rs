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
    };

    use ili9341::{DisplaySize240x320, Ili9341, Orientation};
    use stm32l4xx_hal::{
        self,
        delay::DelayCM,
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

        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut pwr = cx.device.PWR.constrain(&mut rcc.apb1r1);
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb2);

        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        /*
         *  The ILI9341 driver
         */
        // Configure SPI
        let lcd_clk = gpiob.pb13.into_alternate(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpiob.afrh,
        );

        let lcd_miso = gpiob.pb14.into_alternate(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpiob.afrh,
        );

        let lcd_mosi = gpiob.pb15.into_alternate(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpiob.afrh,
        );

        let lcd_cs = gpiob
            .pb12
            .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);

        let lcd_spi = Spi::spi2(
            cx.device.SPI2,
            (lcd_clk, lcd_miso, lcd_mosi),
            MODE,
            16.MHz(),
            clocks,
            &mut rcc.apb1r1,
        );
        
        // let lcd_clk = gpiob.pb0.into_alternate();
        // let lcd_miso = NoMiso {};
        // let lcd_mosi: stm32l4xx_hal::gpio::Pin<'A', 10, stm32l4xx_hal::gpio::Alternate<6>> = gpioa.pa10.into_alternate().internal_pull_up(true);
        let lcd_dc = gpiob.pb1.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        // let lcd_cs = gpiob.pb2.into_push_pull_output();
        // let mode = Mode {
        //     polarity: Polarity::IdleLow,
        //     phase: Phase::CaptureOnFirstTransition,
        // };
        // let lcd_spi = dp
        //     .SPI5
        //     .spi((lcd_clk, lcd_miso, lcd_mosi), mode, 2.MHz(), &clocks);
        let spi_iface = SPIInterface::new(lcd_spi, lcd_dc, lcd_cs);
        let dummy_reset = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let mut delay = DelayCM::new(clocks);
        let mut lcd = Ili9341::new(
            spi_iface,
            dummy_reset,
            &mut delay,
            Orientation::PortraitFlipped,
            DisplaySize240x320,
        )
        .unwrap();

        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

        // Create a text at position (20, 30) and draw it using the previously defined style
        Text::with_alignment(
            "First line",
            Point::new(20, 30),
            style,
            Alignment::Center,
        )
        .draw(&mut lcd)
        .unwrap();

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