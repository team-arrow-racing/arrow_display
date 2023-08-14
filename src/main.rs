use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

pub mod speed_display;
use speed_display::SpeedDisplay;
use std::thread;
use std::time::Duration;
use embedded_graphics::prelude::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use rand;

fn main() {
    let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(320, 240));
    let mut speed_display = SpeedDisplay::new(display);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Speed Display", &output_settings);

    loop {
        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                return;
            }
        }

        let speed = get_current_speed();
        if let Err(e) = speed_display.show_speed(speed) {
            eprintln!("Error displaying speed: {}", e);
        }

        window.update(&speed_display.display);
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_current_speed() -> u32 {
    // Here you can implement the logic to get the current speed.
    // For now, we'll just return a random number.
    rand::random::<u32>() % 100
}
