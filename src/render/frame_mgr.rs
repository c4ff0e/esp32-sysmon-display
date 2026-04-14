use crate::{render, sound::beep};
use crate::usb::data::IncomingMetrics;
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::{Point, *}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment}, text::Text};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::Output;
use esp_hal::delay::Delay;
use esp_hal::spi::master::Spi;
use profont::PROFONT_12_POINT;
use crate::render::{message};
use core::fmt::Write;
use heapless::String;

const CPU_BORDER: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()                                       
    .stroke_color(Rgb565::CSS_STEEL_BLUE)                                         
    .stroke_width(2)                                                              
    .stroke_alignment(StrokeAlignment::Inside)                                    
    .build();

const GPU_BORDER: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()                                       
    .stroke_color(Rgb565::CSS_ORANGE_RED)                                         
    .stroke_width(2)                                                              
    .stroke_alignment(StrokeAlignment::Inside)                                    
    .build();

const RAM_BORDER: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()                                       
    .stroke_color(Rgb565::CSS_DARK_GREEN)                                         
    .stroke_width(2)                                                              
    .stroke_alignment(StrokeAlignment::Inside)                                    
    .build();

const CPU_TEXT_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&PROFONT_12_POINT, Rgb565::CSS_STEEL_BLUE);
const GPU_TEXT_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&PROFONT_12_POINT, Rgb565::CSS_ORANGE_RED);
const RAM_TEXT_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&PROFONT_12_POINT, Rgb565::CSS_DARK_GREEN);


pub fn all_unsupported(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        message::draw(display, "Your device\nis unsupported", Point::new(80, 54));
        beep::all_unsupported_beep(beeper, delay);
}

pub fn no_metrics(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        message::draw(display, "No data\nCheck server", Point::new(80, 54));
        beep::no_metrics_beep(beeper, delay);
}

pub fn connect_usb(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        message::draw(display, "You are\nconnected to\nCOM port\nSwitch port",Point::new(80, 34));
        beep::connect_usb_beep(beeper, delay);
    }

pub fn message_cpu(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        message::draw(display, "Your CPU\nis not\nsupported",Point::new(80, 44));
        beep::connect_usb_beep(beeper, delay);
    }

pub fn message_gpu(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        message::draw(display, "Your GPU\nis not\nsupported",Point::new(80, 44));
        beep::connect_usb_beep(beeper, delay);
    }

pub fn full_initial(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    incoming_metrics: &IncomingMetrics
    ){  
        //TODO: move this to common function
        // there is enough ram to allocate 32/64 bits for strings

        // cpu
        //TODO: it is possible to also render cpu name, but cpu name needs to be somehow formatted
        // e.g full vendor string -> Core i5 12400f
        let mut cpu_string: String<32> = String::new(); 
        let _ = write!(&mut cpu_string, "CPU: {:.0}%, {} MHz",incoming_metrics.cpu_usage, incoming_metrics.cpu_frequency);
        let cpu_text = Text::new(cpu_string.as_str(), Point { x: 5, y: 25 }, CPU_TEXT_STYLE);

        // gpu
        // get mem usage %
        let mem_pct = if incoming_metrics.gpu_memory_total > 0 {                          
            incoming_metrics.gpu_memory_used * 100 / incoming_metrics.gpu_memory_total    
        } else {                                                                          
            0                                                                             
        };

        let mut gpu_string: String<64> = String::new();
        let _ = write!(&mut gpu_string, "GPU: {:.0}%, {}°C\n{}MHz, VRAM {}%", incoming_metrics.gpu_usage, incoming_metrics.gpu_temp, incoming_metrics.gpu_freq, mem_pct, ); 
        let gpu_text = Text::new(gpu_string.as_str(), Point { x: 5, y: 60 }, GPU_TEXT_STYLE);

        // ram
        let mut ram_string: String<64> = String::new();
        let _ = write!(&mut ram_string, "RAM TOTAL: {} GB\nRAM USED: {} GB", incoming_metrics.total_ram / (1024 * 1024 * 1024), incoming_metrics.used_ram / (1024 * 1024 * 1024));
        // for me ram text appears offsetted in comparison with gpu text. maybe it is bacuse you cant divide screen into three equal regions
        let ram_text = Text::new(ram_string.as_str(), Point { x: 5, y: 102 }, RAM_TEXT_STYLE);
        render::metrics::full_initial(display, CPU_BORDER, GPU_BORDER, RAM_BORDER, cpu_text, gpu_text, ram_text);  
}