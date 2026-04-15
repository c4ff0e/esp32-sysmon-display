use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::*, primitives::{PrimitiveStyle, Rectangle}, text::Text
};
use heapless::String;
use core::fmt::Write;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{gpio::Output, spi::master::Spi};

pub fn create_cpu_text<'text, 'style>(
    cpu_name: &str,
    cpu_usage: f32,
    cpu_frequency: u32,
    cpu_text: &'text mut String<64>, 
    cpu_text_style: MonoTextStyle<'style, Rgb565>,
    position: Point
    ) -> Text<'text, MonoTextStyle<'style, Rgb565>>
    {   
        let _ = write!(cpu_text, "{}\n{:>4}MHz {:>3}% ",cpu_name, cpu_frequency, cpu_usage as u32);
        Text::new(cpu_text.as_str(), position, cpu_text_style)
}

pub fn create_gpu_text<'text, 'style>(
    gpu_usage: f32, 
    gpu_temp: u32, 
    gpu_freq: u32, 
    gpu_mem_pct: u64, 
    gpu_text: &'text mut String<64>, 
    gpu_text_style: MonoTextStyle<'style, Rgb565>,
    position: Point
    ) -> Text<'text, MonoTextStyle<'style, Rgb565>>
    {
    let _ = write!(gpu_text, "GPU: {:>3}% {:>3}°C\n{:>4}MHz VRAM{:>3}% ", gpu_usage as u32, gpu_temp, gpu_freq, gpu_mem_pct, ); 
    Text::new(gpu_text.as_str(), position, gpu_text_style)
}

pub fn create_ram_text<'text, 'style>(
    total_ram: u64,
    used_ram: u64,
    ram_text: &'text mut String<64>, 
    ram_text_style: MonoTextStyle<'style, Rgb565>,
    position: Point
    ) -> Text<'text, MonoTextStyle<'style, Rgb565>>
    {
        let _ = write!(ram_text, "RAM TOTAL:{:>3} GB\nRAM USED:{:>3} GB", total_ram / (1024 * 1024 * 1024), used_ram / (1024 * 1024 * 1024));
        // for me ram text appears offsetted in comparison with gpu text. maybe it is bacuse you cant divide screen into three equal regions
        Text::new(ram_text.as_str(), position, ram_text_style)
}

pub fn full_initial(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    cpu_border: PrimitiveStyle<Rgb565>,
    gpu_border: PrimitiveStyle<Rgb565>,
    ram_border: PrimitiveStyle<Rgb565>,
    cpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    gpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
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
    cpu_zone.draw(display).unwrap();
    cpu_text.draw(display).unwrap();
    
    gpu_zone.draw(display).unwrap();
    gpu_text.draw(display).unwrap();

    ram_zone.draw(display).unwrap();
    ram_text.draw(display).unwrap();
}
pub fn unsupported_cpu_initial(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    gpu_border: PrimitiveStyle<Rgb565>,
    ram_border: PrimitiveStyle<Rgb565>,
    gpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
){
    // create boxes
    let gpu_zone = Rectangle::new(Point::new(0, 0), Size::new(160, 64))
        .into_styled(gpu_border);
    let ram_zone = Rectangle::new(Point::new(0, 64), Size::new(160, 64))
        .into_styled(ram_border);

    // blank the screen
    display.clear(Rgb565::BLACK).unwrap();

    // draw
    gpu_zone.draw(display).unwrap();
    gpu_text.draw(display).unwrap();

    ram_zone.draw(display).unwrap();
    ram_text.draw(display).unwrap();
}


pub fn unsupported_gpu_initial(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    cpu_border: PrimitiveStyle<Rgb565>,
    ram_border: PrimitiveStyle<Rgb565>,
    cpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
){
    // create boxes
    let cpu_zone = Rectangle::new(Point::new(0, 0), Size::new(160, 64))
        .into_styled(cpu_border);
    let ram_zone = Rectangle::new(Point::new(0, 64), Size::new(160, 64))
        .into_styled(ram_border);

    // blank the screen
    display.clear(Rgb565::BLACK).unwrap();

    // draw
    cpu_zone.draw(display).unwrap();
    cpu_text.draw(display).unwrap();

    ram_zone.draw(display).unwrap();
    ram_text.draw(display).unwrap();
}
// dirty funcitons rerender only text with black background to decrease shimmering
pub fn dirty_full(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    cpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    gpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
){

    //draw
    cpu_text.draw(display).unwrap();

    gpu_text.draw(display).unwrap();

    ram_text.draw(display).unwrap();
}

pub fn dirty_unsupported_cpu(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    gpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>
){
    gpu_text.draw(display).unwrap();

    ram_text.draw(display).unwrap();
}

pub fn dirty_unsupported_gpu(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    cpu_text: Text<'_, MonoTextStyle<'_, Rgb565>>,
    ram_text: Text<'_, MonoTextStyle<'_, Rgb565>>
){
    cpu_text.draw(display).unwrap();

    ram_text.draw(display).unwrap();
}