use crate::render::common::FrameKind;
use crate::sound::beep;
use esp_hal::gpio::Output;
use esp_hal::delay::Delay;
use log::info;
fn increment(frames: &mut Option<i32>) {
    match frames {
        Some(frames) => *frames += 1,
        None => return,
    }
}

//TODO:rendering code, return type

//if this function gets kind:GpuAndCpu, frame count does not go up => renders on every pass
//if frames >10: unsupported frames will not render

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
        FrameKind::GpuAndCpu => {
            //rendering code

            //
            // beep one time
            match unsupported_beep{
                true => {}
                false => {
                    beep::all_unsupported_beep(beeper, delay);
                    *unsupported_beep = true;
                    info!("gpu+cpu unsupported beep");
                }
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
