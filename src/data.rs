use postcard::fixint::le;
use serde::Deserialize;
use log::info;

#[derive(Deserialize)]
pub struct IncomingMetrics<'a>{
    pub cpu_usage: f32,
    pub cpu_name: &'a str,
    pub cpu_frequency: u32,
    pub cpu_is_supported: bool,

    pub total_ram: u64,
    pub used_ram: u64,

    pub gpu_name: &'a str,
    pub gpu_usage: f32,
    pub gpu_temp: u32,
    pub gpu_memory_total: u64,
    pub gpu_memory_used: u64,
    pub gpu_freq: u32,
    pub gpu_supported: bool,

}

pub struct DeviceState{
    cpu_name: heapless::String<128>,
    cpu_supported: bool,

    gpu_name: heapless::String<128>,
    gpu_supported:bool,

    total_ram: u64,
    gpu_memory_total: u64,
}

impl DeviceState{
    pub fn new(incoming: &IncomingMetrics<'_>)-> Self {

        let mut cpu_name = heapless::String::<128>::new();
        cpu_name.push_str(incoming.cpu_name).unwrap_or_else(|_| {
            info!("Failed to set CPU name: input string is too long");
        });

        let mut gpu_name = heapless::String::<128>::new();
        gpu_name.push_str(incoming.gpu_name).unwrap_or_else(|_| {
            info!("Failed to set GPU name: input string is too long");
        });
        Self{
            cpu_name,
            cpu_supported: incoming.cpu_is_supported,

            gpu_name,
            gpu_supported: incoming.gpu_supported,

            total_ram: incoming.total_ram / (1024 * 1024 * 1024), // Convert bytes to GB
            gpu_memory_total: incoming.gpu_memory_total / (1024 * 1024 * 1024), // Convert bytes to GB
        }
    }
}