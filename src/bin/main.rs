#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use core::ptr::addr_of_mut;
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
use display::render;
use display::render::common::FrameKind;

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::time::Instant;
use esp_hal::{
    main,
    otg_fs::{Usb, UsbBus},
};

use log::info;

use usb_device::device::StringDescriptors;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use postcard::accumulator::{CobsAccumulator, FeedResult};

use display::data::{DeviceState, IncomingMetrics};
use display::receive;

static mut EP_MEMORY: [u32; 1024] = [0; 1024]; // 4KB of memory for USB endpoints

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    let usb_bus = UsbBus::new(usb, unsafe { &mut *addr_of_mut!(EP_MEMORY) });

    let mut serial = SerialPort::new(&usb_bus); // usb serial port

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x303A, 0x3001))
        .strings(&[StringDescriptors::default()
            .manufacturer("Gadza Techonologies")
            .product("Zhmishenko Valeriy Albertovich")
            .serial_number("Dve dvoechki odna vos'merochka")])
        .expect("Failed to set USB device strings")
        .device_class(USB_CLASS_CDC)
        .build();

    let mut accumulator = CobsAccumulator::<256>::new();
    let mut rx_buf = [0u8; 64];

    let mut device_state: Option<DeviceState> = None;
    let mut current_metrics: Option<IncomingMetrics> = None;

    let mut unsupported_frames = Some(0);
    loop {
        // main
        let pipeline_start = Instant::now();

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // reading serial input
        let received_bytes = receive::receive_data(&mut serial, &mut rx_buf);
        match received_bytes {
            Ok(count) if count > 0 => {
                let mut chunk = &rx_buf[..count];

                //feed accumulor till its full
                while !chunk.is_empty() {
                    let accumulator_result = receive::accumulate(&mut accumulator, chunk);
                    match accumulator_result {
                        FeedResult::Consumed => break,
                        FeedResult::Success { data, remaining } => {
                            chunk = remaining;
                            if device_state.is_none() {
                                device_state = Some(DeviceState::new(&data));
                            }
                            current_metrics = Some(data);
                        }
                        FeedResult::OverFull(remaining) => {
                            chunk = remaining;
                            info!("Accumulator overflow!");
                        }
                        FeedResult::DeserError(remaining) => {
                            chunk = remaining;
                            info!("Failed to deserialize data!");
                        }
                    }
                }
            }
            Ok(_) => {
                // no data
            }
            Err(usbd_serial::UsbError::WouldBlock) => {
                // no data
            }
            Err(e) => {
                info!("Error receiving data: {:?}", e);
            }
        }

        // render starts here
        if let (Some(device_state), Some(current_metrics)) = (&device_state, &current_metrics) {
            // this eats a lot of time
            /*
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
            */

            // render unsupported continuosly
            if !device_state.cpu_supported && !device_state.gpu_supported {
                render::unsupported::render_unsupported(&mut None, FrameKind::GpuAndCpu)
            }
            // render cpu unsupported frames + cpu-only layout
            else if !device_state.cpu_supported && device_state.gpu_supported {
                render::unsupported::render_unsupported(&mut unsupported_frames,FrameKind::Cpu,)
            }
            // render gpu unsupported frames + gpu-only layout
            else if device_state.cpu_supported && !device_state.gpu_supported {
                render::unsupported::render_unsupported(&mut unsupported_frames,FrameKind::Gpu,)
            }
            // render everything (CPU TEMP IS RENDERED ALWAYS; IN CASE OF CPU TEMP == 0.0 RENDER X MARK)
            else {
                //both renders
            }
        }

        let pipeline_duration = pipeline_start.elapsed();
        info!(
            "Pipeline execution time: {:?} ms",
            pipeline_duration.as_millis()
        );
    }
}
