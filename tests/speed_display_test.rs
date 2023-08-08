use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
};

use arrow_display::speed_display::SpeedDisplay;

#[test]
fn test_show_speed() {
    let display = MockDisplay::new();
    let mut speed_display = SpeedDisplay::new(display.clone());

    let result = speed_display.show_speed(60);

    assert!(result.is_ok(), "Error displaying speed: {:?}", result.unwrap_err());

    // Check if the rectangle is drawn correctly
    let expected_display = MockDisplay::from_pattern(&[
        "################",
        "#              #",
        "#              #",
        "#     60       #",
        "#              #",
        "#              #",
        "################",
    ]);

    assert_eq!(
        display,
        expected_display,
        "Display does not match expected display"
    );
}