use log::{info};
use postcard::accumulator::{CobsAccumulator, FeedResult};
use usbd_serial::SerialPort;
use esp_hal::{
    otg_fs::{Usb, UsbBus} };

use crate::data::IncomingMetrics;



pub fn receive_data(serial: &mut SerialPort<'_, UsbBus<Usb<'_>>>, rx_buf: &mut [u8]) -> Result<usize, usbd_serial::UsbError> {
    let incoming = serial.read(rx_buf);
    match incoming {
        // successfully received data
        Ok(count) if count > 0 => {
            Ok(count)
        },
        // no data received
        Ok(_) => {
            Err(usbd_serial::UsbError::WouldBlock)
        },
        Err(e) => {
            info!("Error reading from serial port: {:?}", e);
            Err(e)
        }
    }
}

pub fn accumulate<'a>(accumulator: &mut CobsAccumulator<256>, chunk: &'a [u8]) -> FeedResult<'a, IncomingMetrics> {
accumulator.feed::<IncomingMetrics>(chunk)
}