#![no_std]

use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
};

use arrow_display::speed_display::SpeedDisplay;

#[test]
fn test_show_speed() {
    // 240 * 320
    let display = MockDisplay::new();
    let mut speed_display = SpeedDisplay::new(display);

    let result = speed_display.show_speed(120);

    assert!(result.is_ok(), "Error displaying speed: {:?}", result.unwrap_err());

}