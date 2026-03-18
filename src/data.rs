use heapless::String;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
// it is kinda wasteful to save strings value on every succesfull packet, but anyway...
pub struct IncomingMetrics {
    pub cpu_usage: f32,
    pub cpu_name: String<128>,
    pub cpu_frequency: u32,
    pub cpu_is_supported: bool,
    pub cpu_temp: f32,

    pub total_ram: u64,
    pub used_ram: u64,

    pub gpu_name: String<128>,
    pub gpu_usage: f32,
    pub gpu_temp: u32,
    pub gpu_memory_total: u64,
    pub gpu_memory_used: u64,
    pub gpu_freq: u32,
    pub gpu_supported: bool,
}

pub struct DeviceState {
    pub cpu_name: heapless::String<128>,
    pub cpu_supported: bool,
    pub cpu_temp_supported:bool,

    pub gpu_name: heapless::String<128>,
    pub gpu_supported: bool,

    pub total_ram: u64,
    pub gpu_memory_total: u64,
}

impl DeviceState {
    pub fn new(incoming: &IncomingMetrics) -> Self {
        let mut cpu_name = heapless::String::<128>::new();
        cpu_name.push_str(&incoming.cpu_name).unwrap_or_else(|_| {
            info!("Failed to set CPU name: input string is too long");
        });

        let mut gpu_name = heapless::String::<128>::new();
        gpu_name.push_str(&incoming.gpu_name).unwrap_or_else(|_| {
            info!("Failed to set GPU name: input string is too long");
        });

        // TODO: fix deciding this only on the first successful packet
        let cpu_temp_supported = incoming.cpu_is_supported && incoming.cpu_temp != 0.0;

        Self {
            cpu_name,
            cpu_supported: incoming.cpu_is_supported,
            cpu_temp_supported,

            gpu_name,
            gpu_supported: incoming.gpu_supported,

            total_ram: incoming.total_ram / (1024 * 1024 * 1024), // Convert bytes to GB
            gpu_memory_total: incoming.gpu_memory_total / (1024 * 1024 * 1024), // Convert bytes to GB
        }
    }
}
