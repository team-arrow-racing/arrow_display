use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::Text,
};

pub struct SpeedDisplay<DI>
where
    DI: DrawTarget<Color = BinaryColor>,
{
    pub display: DI,
}

impl<DI> SpeedDisplay<DI>
where
    DI: DrawTarget<Color = BinaryColor>,
{
    pub fn new(display: DI) -> Self {
        SpeedDisplay { display }
    }

    pub fn show_speed(&mut self, speed: u32) -> Result<(), DI::Error> {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        // Add km/h to the speed
        let binding = speed.to_string() + " km/h";

        let speed_text = Text::new(&binding, Point::new(15, 28), text_style);


        Rectangle::new(Point::new(0, 0), Size::new(64, 64))
            .into_styled(style)
            .draw(&mut self.display)?;

        speed_text.draw(&mut self.display)?;

        Ok(())
    }
}
