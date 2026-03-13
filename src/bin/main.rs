#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

#![deny(clippy::large_stack_frames)]
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

use core::ptr::addr_of_mut;

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::time::{Duration, Instant};
use esp_hal::{
    main,
    otg_fs::{Usb, UsbBus} };

use log::{info};

use usb_device::device::StringDescriptors;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};

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

    loop { // main
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        info!("USB device polled successfully");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
