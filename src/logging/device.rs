use crate::usb::data::{DeviceState, IncomingMetrics};
use log::info;
pub fn metrics(current_metrics: &IncomingMetrics, device_state: &DeviceState){
    info!(
    "Device State: CPU: {} (Supported: {}), GPU: {} (Supported: {}), Total RAM: {} GB, GPU Memory Total: {} GB",
    device_state.cpu_name,
    device_state.cpu_supported,
    device_state.gpu_name,
    device_state.gpu_supported,
    device_state.total_ram,
    device_state.gpu_memory_total
    );
    let ram_used_gb = current_metrics.used_ram / (1024 * 1024 * 1024);
    let gpu_memory_used_gb = current_metrics.gpu_memory_used / (1024 * 1024 * 1024);

    info!(
        "CPU | load {:>5.2}% | freq {:>4} MHz | temp {:>4.1} C | RAM {:>2}/{} GB",
        current_metrics.cpu_usage,
        current_metrics.cpu_frequency,
        current_metrics.cpu_temp,
        ram_used_gb,
        device_state.total_ram
        );
        info!(
        "GPU | load {:>5.2}% | freq {:>4} MHz | temp {:>3} C | VRAM {:>2}/{} GB",
        current_metrics.gpu_usage,
        current_metrics.gpu_freq,
        current_metrics.gpu_temp,
        gpu_memory_used_gb,
        device_state.gpu_memory_total
        );
            
}
