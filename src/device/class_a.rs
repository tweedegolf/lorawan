use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

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
pub struct ClassA<RXTX, TIM, RNG, ERR, R>(Device<RXTX, TIM, RNG, ERR, DeviceState<R>>);

impl<RXTX, TIM, RNG, ERR, INFO, CH, R> ClassA<RXTX, TIM, RNG, ERR, R>
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
    R: Region,
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(&mut self, tx: &[u8], rx: &mut [u8]) -> TransmitResult<RXTX, TIM, RNG, ERR> {
        let uplink = Uplink::new(tx, 1, &mut self.state)?;
        let downlink = self.0.radio.lorawan_transmit(
            uplink.as_bytes(),
            rx,
            self.0.state.tx_dr(),
            &self.0.state.settings(),
        )?;

        match downlink {
            None => Ok(None),
            Some((n, info)) => {
                let downlink = Downlink::from_data(&mut rx[..n], &mut self.state)?;
                rx.copy_from_slice(downlink.as_bytes());
                Ok(Some((n, info)))
            }
        }
    }
}

impl<RXTX, TIM, RNG, ERR, R> From<Device<RXTX, TIM, RNG, ERR, DeviceState<R>>>
    for ClassA<RXTX, TIM, RNG, ERR, R>
{
    fn from(device: Device<RXTX, TIM, RNG, ERR, DeviceState<R>>) -> Self {
        ClassA(device)
    }
}

impl<RXTX, TIM, RNG, ERR, R> Deref for ClassA<RXTX, TIM, RNG, ERR, R> {
    type Target = Device<RXTX, TIM, RNG, ERR, DeviceState<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<RXTX, TIM, RNG, ERR, R> DerefMut for ClassA<RXTX, TIM, RNG, ERR, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
