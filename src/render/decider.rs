use crate::render::common::{FrameKind, RenderDecision};
use crate::usb::data::DeviceState;

pub fn decider(device_state: &DeviceState) -> RenderDecision {
    //gpu unsupported
    if device_state.cpu_supported && !device_state.gpu_supported {
        return RenderDecision::Unsupported(FrameKind::Gpu);
    }
    //cpu unsupported
    else if !device_state.cpu_supported && device_state.gpu_supported {
        return RenderDecision::Unsupported(FrameKind::Cpu);
    }
    //everything unsupported
    else if !device_state.cpu_supported && !device_state.gpu_supported {
        return RenderDecision::Unsupported(FrameKind::GpuAndCpu);
    }
    // everything is supported
    else {
        return RenderDecision::Full;
    }
}
