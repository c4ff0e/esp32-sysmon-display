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

use esp_backtrace as _;
use esp_hal::{
    main,
    otg_fs::{Usb, UsbBus},
    gpio::{Output, OutputConfig, Level},
    time::Instant,
    clock::CpuClock,
    delay::Delay
};

use log::info;

use usb_device::device::StringDescriptors;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use postcard::accumulator::{CobsAccumulator,};

use display::usb::data::{DeviceState, IncomingMetrics};
use display::usb::receive;

use display::render;
use display::render::common::{RenderDecision, FrameKind};

use display::logging;

// 4KB of memory for USB endpoints
static mut EP_MEMORY: [u32; 1024] = [0; 1024];

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

    //state
    let mut device_state: Option<DeviceState> = None;
    let mut incoming_metrics: Option<IncomingMetrics> = None;

    let mut unsupported_beep = false;
    let mut no_metrics_beep = false;
    //frames
    let mut unsupported_frames_count = Some(0);
    const MAX_UNSUPPORTED_FRAMES: i32 = 10; //can be changed

    //beeper
    let mut beeper = Output::new(
        peripherals.GPIO4,
        Level::Low,
        OutputConfig::default(),
    );

    //blocking delay for beeps
    let delay = Delay::new();

    loop {
        // main
        let pipeline_start = Instant::now();

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // reading serial input
        let received_bytes = receive::receive_data(&mut serial, &mut rx_buf);
        receive::process_received(received_bytes, &rx_buf, &mut accumulator, &mut device_state, &mut incoming_metrics);

        // render starts here
        if let (Some(device_state), Some(incoming_metrics)) = (&device_state, &incoming_metrics) {
            // this eats a lot of time
            //logging::device::metrics(&incoming_metrics, &device_state);

            match render::decider::decider(&device_state) {
                RenderDecision::Unsupported(kind) => {
                    render::unsupported::render_unsupported(
                        &mut unsupported_frames_count,
                        kind,
                        MAX_UNSUPPORTED_FRAMES,
                        &mut beeper,
                        &delay,
                        &mut unsupported_beep,
                        &mut no_metrics_beep
                    );
                }

                RenderDecision::Full => {
                    // renders everything
                }
            }
        }
        else {
            render::unsupported::render_unsupported(
                &mut unsupported_frames_count,
                FrameKind::NoMetrics,
                MAX_UNSUPPORTED_FRAMES,
                &mut beeper, 
                &delay,
                &mut unsupported_beep,
                &mut no_metrics_beep
            );
        }

        let pipeline_duration = pipeline_start.elapsed();
        info!(
            "Pipeline execution time: {:?} ms",
            pipeline_duration.as_millis()
        );
    }
}
