use crate::render::common::FrameKind;
fn increment(frames: &mut Option<i32>){
    match frames {
        Some(frames) => {
            *frames += 1
        }
        None => return
    }
}

//TODO:rendering code, return type

//if this function gets kind:GpuAndCpu, frame count does not go up => renders on every pass
//if frames >10 (currently inline number): unsupported frames will not render

pub fn render_unsupported(frames: &mut Option<i32>, kind: FrameKind, max_unsupported_frames:i32) {
    match kind {

        FrameKind::Cpu => {
            if frames.is_none() || frames.is_some_and(|n| n < max_unsupported_frames){
                //rendering code
            
                //

                //count up frame
                increment(frames);
            }
            //counter is over frame count
            else {
                return
            }
        }

        FrameKind::Gpu => {
            if frames.is_some_and(|n| n < max_unsupported_frames){
                //rendering code
            
                //

                //count up frame
                increment(frames);
            }
            //counter is over frame count
            else {
                return
            }
        }
        
        // frame count does not go up
        FrameKind::GpuAndCpu => {
                //rendering code
            
                //

            }

        }
    }

