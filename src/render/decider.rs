use crate::render::common::{FrameKind, RenderDecision};
use crate::usb::data::{DeviceState, IncomingMetrics};
use usb_device::device::{UsbDeviceState};

fn increment(frames: &mut i32) {
    *frames += 1;
    }

// it is a bit wasteful to copy entire struct but performance damage is negligable
fn check_staleness(
    incoming_metrics: &Option<IncomingMetrics>,
    incoming_prev: &mut Option<IncomingMetrics>,
    metrics_stale_counter: &mut i32,
    max_stale_frames: i32,
) -> bool
{
    if incoming_metrics == incoming_prev{
        if *metrics_stale_counter < max_stale_frames{
            *metrics_stale_counter +=1;
            return false
        }
        else{
            return true;
        }
    }
    else{
        *incoming_prev = incoming_metrics.clone();
        *metrics_stale_counter = 0;
        return false
    }
}

pub fn decider(
    incoming_metrics: &Option<IncomingMetrics>,
    device_state: &Option<DeviceState>, 
    usb_state: UsbDeviceState, 
    unsupported_frames_count: &mut i32,
    incoming_prev: &mut Option<IncomingMetrics>,
    metrics_stale_counter: &mut i32,
    max_stale_frames: i32,
    max_unsupported_frames: i32
) -> RenderDecision {
    let frames_remaining = *unsupported_frames_count < max_unsupported_frames;
    let metrics_stale = check_staleness(incoming_metrics, incoming_prev, metrics_stale_counter, max_stale_frames);

    match device_state { 
        Some(device_state) => 
        {
            match (device_state.cpu_supported, device_state.gpu_supported, usb_state == UsbDeviceState::Configured, frames_remaining, metrics_stale) {
                (_, _, false, _, _) => RenderDecision::ConnectUsb,
                (_, _, true, _, true) => RenderDecision::NoMetrics,
                (true, false, true, false, false) => {
                    RenderDecision::Unsupported(FrameKind::Gpu)
                }
                (false, true, true, false, false) => {
                    RenderDecision::Unsupported(FrameKind::Cpu)
                }
                (false, false, true, _, _) => {
                    RenderDecision::Unsupported(FrameKind::GpuAndCpu)
                }
                (true, true, true, _, false) => {
                    RenderDecision::Full
                }
                (true, false, true, true, _) => { increment(unsupported_frames_count); RenderDecision::MessageGpu },
                (false, true, true, true, _) => { increment(unsupported_frames_count); RenderDecision::MessageCpu },
                
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