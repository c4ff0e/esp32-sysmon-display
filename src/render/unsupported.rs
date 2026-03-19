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
            if frames.is_some_and(|n| n < 10){
                //rendering code
            
                //

                //count up frame
                increment(frames);
            }
            //stop rendering frames
            else {
                return
            }
        }
        FrameKind::Gpu => {
            if frames.is_some_and(|n| n < 10){
                //rendering code
            
                //

                //count up frame
                increment(frames);
            }
            //stop rendering frames
            else {
                return
            }
        }
        FrameKind::GpuAndCpu => {
            if frames.is_some_and(|n| n < 10){
                //rendering code
            
                //

                //count up frame
                increment(frames);
            }
            //stop rendering frames
            else {
                return
            }
        }
    };
}
