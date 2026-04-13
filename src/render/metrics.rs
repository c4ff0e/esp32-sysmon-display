use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565, 
    prelude::*, 
    text::{Alignment, Baseline, Text, TextStyleBuilder},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{gpio::Output, spi::master::Spi};
use profont::PROFONT_7_POINT;

pub fn full_initial(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    cpu_border: PrimitiveStyle<Rgb565>,
    gpu_border: PrimitiveStyle<Rgb565>,
    ram_border: PrimitiveStyle<Rgb565>,
    cpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>
){

    // create boxes
    let cpu_zone = Rectangle::new(Point::new(0, 0), Size::new(160, 42))
        .into_styled(cpu_border);
    let gpu_zone = Rectangle::new(Point::new(0, 42), Size::new(160, 42))
        .into_styled(gpu_border);
    let ram_zone = Rectangle::new(Point::new(0, 84), Size::new(160, 44))
        .into_styled(ram_border);

    
    
    // blank the screen
    display.clear(Rgb565::BLACK).unwrap();
    // draw
    cpu_text.draw(display).unwrap();
    cpu_zone.draw(display).unwrap();
    gpu_zone.draw(display).unwrap();
    ram_zone.draw(display).unwrap();
}
/* 
cpu_text_style: MonoTextStyle<'_, Rgb565>,
gpu_text_style: MonoTextStyle<'_, Rgb565>,
ram_text_style: MonoTextStyle<'_, Rgb565>,
*/