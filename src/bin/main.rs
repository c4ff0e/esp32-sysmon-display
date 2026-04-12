#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use core::ptr::addr_of_mut;
use embedded_hal_bus::spi::ExclusiveDevice;
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay, 
    gpio::{Level, Output, OutputConfig}, 
    main, 
    otg_fs::{Usb, UsbBus},
    time::Instant,
    spi::{Mode, master::{Spi, Config}},
    time::Rate
};

use st7735_lcd;
use st7735_lcd::Orientation;

use embedded_graphics::{
    pixelcolor::Rgb565, prelude::*,
};

use log::info;

use usb_device::device::{StringDescriptors, UsbDeviceState};
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

    let mut delay = Delay::new();

    //display
    let display_config = Config::default()
    .with_frequency(Rate::from_mhz(40)) //currently experimental
    .with_mode(Mode::_0);

    let cs = Output::new(
        peripherals.GPIO1,
        Level::High,
        OutputConfig::default()
    );

    let reset = Output::new(
        peripherals.GPIO2,
        Level::High,
        OutputConfig::default()
    );

    let dc = Output::new(
        peripherals.GPIO7,
        Level::Low,
        OutputConfig::default()
    );
    let spi_bus = Spi::new(peripherals.SPI2, display_config).unwrap()
    .with_sck(peripherals.GPIO12)
    .with_mosi(peripherals.GPIO11);
    
    let spi_dev = ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();

    let width = 160;
    let height = 128;
    let rgb = true;
    let inverted = false;

    let mut display = st7735_lcd::ST7735::new(spi_dev, dc, reset, rgb, inverted, width, height);
    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    
    //beeper
    let mut beeper = Output::new(
        peripherals.GPIO4,
        Level::Low,
        OutputConfig::default(),
    );

    // main
    loop {
        let pipeline_start = Instant::now();

        usb_dev.poll(&mut [&mut serial]);
        let usb_state = usb_dev.state();

        if usb_state == UsbDeviceState::Configured{
            // reading serial input
            let received_bytes = receive::receive_data(&mut serial, &mut rx_buf);
            receive::process_received(received_bytes, &rx_buf, &mut accumulator, &mut device_state, &mut incoming_metrics);
        }
        else { // reset states so decider works properly
            device_state = None;
            incoming_metrics = None;
        }
        info!("{:?}",usb_state);
        // render starts here
        if let (Some(device_state), Some(incoming_metrics)) = (&device_state, &incoming_metrics) {
            
            // this eats a lot of time
            //logging::device::metrics(&incoming_metrics, &device_state);

            match render::decider::decider(&device_state,usb_state) {
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
                RenderDecision::ConnectUsb => {
                    render::messages::connect::connect_usb(&mut display);
                }
            }
        }
        else {
            match usb_state == UsbDeviceState::Configured{
                false => {
                    render::messages::connect::connect_usb(&mut display);
                }
                true => {
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
            }

        }

        let pipeline_duration = pipeline_start.elapsed();
        info!(
            "Pipeline execution time: {:?} ms",
            pipeline_duration.as_millis()
        );
    }
}
