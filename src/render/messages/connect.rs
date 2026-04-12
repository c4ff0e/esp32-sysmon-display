use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::*, text::{Alignment, Baseline, Text, TextStyleBuilder}
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{gpio::Output, spi::master::Spi};
use profont::PROFONT_14_POINT;

pub fn connect_usb(display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>){
    let label_style = TextStyleBuilder::new().alignment(Alignment::Center).baseline(Baseline::Middle).build();
    let text_style = MonoTextStyle::new(&PROFONT_14_POINT, Rgb565::WHITE);
    Text::with_text_style(
        "Connect USB",
        Point::new(80, 64),
        text_style, 
        label_style)
        .draw(display)
        .unwrap();

}