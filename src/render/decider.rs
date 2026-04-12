use crate::render::common::{FrameKind, RenderDecision};
use crate::usb::data::DeviceState;
use usb_device::device::{UsbDeviceState};

fn increment(frames: &mut i32) {
    *frames += 1;
    }

pub fn decider(device_state: &Option<DeviceState>, usb_state: UsbDeviceState, max_unsupported_frames: i32, unsupported_frames_count: &mut i32) -> RenderDecision {
    let frames_remaining = *unsupported_frames_count < max_unsupported_frames;
    match device_state { 
        Some(device_state) => 
        {
            match (device_state.cpu_supported, device_state.gpu_supported, usb_state == UsbDeviceState::Configured, frames_remaining) {
                (_, _, false, _) => RenderDecision::ConnectUsb,
                (true, false, true, false) => RenderDecision::Unsupported(FrameKind::Gpu),
                (false, true, true, false) => RenderDecision::Unsupported(FrameKind::Cpu),
                (false, false, true, _) => RenderDecision::Unsupported(FrameKind::GpuAndCpu),
                (true, true, true, _) => RenderDecision::Full,
                (true, false, true, true) => { increment(unsupported_frames_count); RenderDecision::MessageGpu },
                (false, true, true, true) => { increment(unsupported_frames_count); RenderDecision::MessageCpu },
                
            }
        }
        None => {
            if usb_state == UsbDeviceState::Configured {
                RenderDecision::NoMetrics
            }
            else {
                RenderDecision::ConnectUsb
            }
        }
    }
    }