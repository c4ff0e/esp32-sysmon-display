use crate::{render, sound::beep};
use crate::usb::data::IncomingMetrics;
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::{Point, *}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment}};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::Output;
use esp_hal::delay::Delay;
use esp_hal::spi::master::Spi;
use profont::PROFONT_12_POINT;
use crate::render::{message};
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
        let mut cpu_text: String<64> = String::new(); 
        let mut gpu_text: String<64> = String::new();
        let mut ram_text: String<64> = String::new();

        // get mem usage %
        let gpu_mem_pct = if incoming_metrics.gpu_memory_total > 0 {                          
            incoming_metrics.gpu_memory_used * 100 / incoming_metrics.gpu_memory_total    
        } else {                                                                          
            0                                                                             
        };

        let cpu_text_pos = Point { x: 5, y: 17 };
        let gpu_text_pos = Point { x: 5, y: 60 };
        let ram_text_pos = Point { x: 5, y: 102 };

        let cpu_text = render::metrics::create_cpu_text(incoming_metrics.cpu_name.as_str(), incoming_metrics.cpu_usage, incoming_metrics.cpu_frequency, &mut cpu_text, CPU_TEXT_STYLE, cpu_text_pos);
        let gpu_text = render::metrics::create_gpu_text(incoming_metrics.gpu_usage, incoming_metrics.gpu_temp, incoming_metrics.gpu_freq, gpu_mem_pct, &mut gpu_text, GPU_TEXT_STYLE, gpu_text_pos);
        let ram_text = render::metrics::create_ram_text(incoming_metrics.total_ram, incoming_metrics.used_ram, &mut ram_text, RAM_TEXT_STYLE, ram_text_pos);

        render::metrics::full_initial(display, CPU_BORDER, GPU_BORDER, RAM_BORDER, cpu_text, gpu_text, ram_text);  
}