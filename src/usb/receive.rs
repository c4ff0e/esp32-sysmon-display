use esp_hal::otg_fs::{Usb, UsbBus};
use log::info;
use postcard::accumulator::{CobsAccumulator, FeedResult};
use usbd_serial::SerialPort;

use crate::usb::data::{IncomingMetrics, DeviceState};

pub fn receive_data(
    serial: &mut SerialPort<'_, UsbBus<Usb<'_>>>,
    rx_buf: &mut [u8],
) -> Result<usize, usbd_serial::UsbError> {
    let incoming = serial.read(rx_buf);
    match incoming {
        // successfully received data
        Ok(count) if count > 0 => Ok(count),
        // no data received
        Ok(_) => Err(usbd_serial::UsbError::WouldBlock),
        Err(e) => {
            Err(e)
        }
    }
}

pub fn accumulate<'a>(
    accumulator: &mut CobsAccumulator<256>,
    chunk: &'a [u8],
) -> FeedResult<'a, IncomingMetrics> {
    accumulator.feed::<IncomingMetrics>(chunk)
}

//god-function
pub fn process_received(
    received_bytes: Result<usize, usb_device::UsbError>,
    rx_buf: &[u8], 
    accumulator: &mut CobsAccumulator<256>, 
    device_state: &mut Option<DeviceState>, 
    incoming_metrics: &mut Option<IncomingMetrics>
    )

    {
    match received_bytes {
        Ok(count) if count > 0 => {
            let mut chunk = &rx_buf[..count];

            //feed accumulor till its full
            while !chunk.is_empty() {
                let accumulator_result = accumulate(accumulator, chunk);
                match accumulator_result {
                    FeedResult::Consumed => break,
                    FeedResult::Success { data, remaining } => {
                        chunk = remaining;
                        if device_state.is_none() {
                            *device_state = Some(DeviceState::new(&data));
                        }
                        *incoming_metrics = Some(data);
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
}