use crate::render::common::{FrameKind, RenderDecision};
use crate::usb::data::DeviceState;
use usb_device::device::{UsbDeviceState};

pub fn decider(device_state: &Option<DeviceState>, usb_state: UsbDeviceState) -> RenderDecision {
    match device_state { 
        Some(device_state) => 
        {
            match (device_state.cpu_supported, device_state.gpu_supported, usb_state == UsbDeviceState::Configured,) {
                (_, _, false) => RenderDecision::ConnectUsb,
                (true, false, true) => RenderDecision::Unsupported(FrameKind::Gpu),
                (false, true, true) => RenderDecision::Unsupported(FrameKind::Cpu),
                (false, false, true) => RenderDecision::Unsupported(FrameKind::GpuAndCpu),
                (true, true, true) => RenderDecision::Full
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