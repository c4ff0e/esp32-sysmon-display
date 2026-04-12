use crate::sound::beep;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::Output;
use esp_hal::delay::Delay;
use esp_hal::spi::master::Spi;
use crate::render::messages;

fn increment(frames: &mut Option<i32>) {
    match frames {
        Some(frames) => *frames += 1,
        None => return,
    }
}
// does not count frames
pub fn all_unsupported(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        messages::unsupported_all::draw(display);
        beep::all_unsupported_beep(beeper, delay);
}

// does not count frames
pub fn no_metrics(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        messages::no_metrics::no_metrics(display);
        beep::no_metrics_beep(beeper, delay);
}

pub fn connect_usb(
    display: &mut st7735_lcd::ST7735<ExclusiveDevice<Spi<'_, esp_hal::Blocking>, Output<'_>, embedded_hal_bus::spi::NoDelay>, Output<'_>, Output<'_>>,
    delay: &Delay,
    beeper: &mut Output<'_>,
    ){
        messages::connect::connect_usb(display);
        beep::connect_usb_beep(beeper, delay);
    }
/* 
pub fn render_unsupported
    (
    frames: &mut Option<i32>, 
    kind: FrameKind, 
    max_unsupported_frames: i32, 
    beeper: &mut Output<'_>, 
    delay: &Delay, 
    unsupported_beep: &mut bool,
    no_metrics_beep: &mut bool
    ) 
    {
    match kind {
        FrameKind::Cpu => {
            if frames.is_none() || frames.is_some_and(|n| n < max_unsupported_frames) {

                //rendering code

                //
                // beep one time
                match unsupported_beep{
                    true => {}
                    false => {
                        beep::cpu_unsupported_beep(beeper, delay);
                        *unsupported_beep = true;
                        info!("cpu unsupported beep");
                    }
                }
                //count up frame
                increment(frames);
            }
            //counter is over frame count
            else {
                return;
            }
        }

        FrameKind::Gpu => {
            if frames.is_some_and(|n| n < max_unsupported_frames) {
                //rendering code

                //
                // beep one time
                match unsupported_beep{
                    true => {}
                    false => {
                        beep::gpu_unsupported_beep(beeper, delay);
                        *unsupported_beep = true;
                        info!("gpu unsupported beep");
                    }
                }
                //count up frame
                increment(frames);
            }
            //counter is over frame count
            else {
                return;
            }
        }

        
        // frame count does not go up
        FrameKind::NoMetrics =>{
            //rendering code

            //
            match no_metrics_beep{
                true => {}
                false => {
                    beep::no_metrics_beep(beeper, delay);
                    *no_metrics_beep = true;
                    info!("no metrics beep");
                }
            }
        }
    }
}
*/