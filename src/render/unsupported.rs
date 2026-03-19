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
pub fn render_unsupported(frames: &mut Option<i32>, kind: FrameKind) {
    match kind {
        FrameKind::Cpu => {
            //rendering code
            
            //

            //count up frame
            increment(frames);
        }
        FrameKind::Gpu => {
            //rendering code

            //

            //count up frame
            increment(frames);
        }
        FrameKind::GpuAndCpu => {
            //rendering code

            //

            //count up frame
            increment(frames);
        }
    };
}
