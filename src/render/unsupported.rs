use crate::render::common::FrameKind;

//TODO:rendering code, return type

//TODO: currently frame counting wouldnt work FIX THIS FIRST
pub fn render_unsupported(frames: &mut Option<i32>, kind:FrameKind){
    match frames{
        Some(frames) => {
            //frames TODO:return type
        }

        None => {
            //doesnt count up
        }
    }
    match kind{
        FrameKind::Cpu => {
            //rendering code
        }
        FrameKind::Gpu => {
            //rendering code
        }
        FrameKind::GpuAndCpu => {
            //rendering code
        }
    };
}