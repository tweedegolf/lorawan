use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::modulation::lora::LoRaChannel;
use radio::{Busy, Channel, Receive, Transmit};
use rand_core::RngCore;

use crate::device::error::DeviceError;
use crate::device::{Device, DeviceState};
use crate::lorawan::{Downlink, Uplink};
use crate::radio::{LoRaInfo, Region};

type TransmitResult<RXTX, TIM, RNG, ERR> =
    Result<Option<(usize, LoRaInfo)>, DeviceError<RXTX, TIM, RNG, ERR>>;

#[derive(Debug)]
pub struct ClassA<RXTX, TIM, RNG, ERR>(Device<RXTX, TIM, RNG, ERR, DeviceState>);

impl<RXTX, TIM, RNG, ERR, INFO, CH> ClassA<RXTX, TIM, RNG, ERR>
where
    RXTX: Receive<Error = ERR, Info = INFO>,
    RXTX: Transmit<Error = ERR>,
    RXTX: Channel<Channel = CH, Error = ERR>,
    RXTX: Busy<Error = ERR>,
    TIM: DelayUs<u32>,
    RNG: RngCore,
    ERR: Debug,
    INFO: Into<LoRaInfo>,
    CH: From<LoRaChannel>,
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
    ) -> TransmitResult<RXTX, TIM, RNG, ERR> {
        let uplink = Uplink::new(tx, 1, &mut self.0.state)?;
        let downlink = self.0.transmit_raw::<R>(uplink.as_bytes(), rx)?;

        match downlink {
            None => Ok(None),
            Some((n, info)) => {
                let downlink = Downlink::from_data(&mut rx[..n], &mut self.0.state)?;
                rx.copy_from_slice(downlink.as_bytes());
                Ok(Some((n, info)))
            }
        }
    }
}

impl<RXTX, TIM, RNG, ERR> From<Device<RXTX, TIM, RNG, ERR, DeviceState>>
    for ClassA<RXTX, TIM, RNG, ERR>
{
    fn from(device: Device<RXTX, TIM, RNG, ERR, DeviceState>) -> Self {
        ClassA(device)
    }
}
