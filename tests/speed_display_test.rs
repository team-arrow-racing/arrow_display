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

    // Check if the rectangle is drawn correctly
    let expected_display = MockDisplay::from_pattern(&[
        "################################################################",
        "#                                                              #",
        // 9
        // 7 TEXT: 120 km/h
        // 48
        "################################################################",
    ]);

    assert_eq!(
        speed_display.display,
        expected_display,
        "Display does not match expected display"
    );
}