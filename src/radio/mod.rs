use core::fmt::Debug;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{BasicInfo, Busy, Channel, RadioState, Receive, ReceiveInfo, Transmit};
use radio::modulation::lora::LoRaChannel;
use rand_core::RngCore;

pub use crate::radio::rate::*;
pub use crate::radio::region::*;

mod rate;
mod region;

pub type Hz = u32;

#[derive(Debug, Default)]
pub struct LoRaState {
    rx_delay: u8,
}

impl LoRaState {
    pub fn set_rx_delay(&mut self, rx_delay: u8) -> &mut Self {
        self.rx_delay = rx_delay;
        self
    }
}

impl RadioState for LoRaState {
    fn idle() -> Self {
        todo!()
    }

    fn sleep() -> Self {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct LoRaInfo {
    rssi: i16,
    snr: i8,
}

impl ReceiveInfo for LoRaInfo {
    fn rssi(&self) -> i16 {
        self.rssi
    }
}

impl From<BasicInfo> for LoRaInfo {
    fn from(info: BasicInfo) -> Self {
        LoRaInfo {
            rssi: info.rssi(),
            snr: 0,
        }
    }
}

/// Combines all the radio traits necessary for LoRa into one trait, and provides useful methods to
/// transmit messages.
pub trait LoRaRadio: Sized {
    /// The time the radio will have to transmit a message before a timeout occurs.
    const TX_TIMEOUT: Duration = Duration::from_millis(4000);

    /// The time the radio will listen for a message on a channel. This must be long enough for the
    /// radio to receive a preamble, in which case it will continue listening for the message. It
    /// must not exceed one second, because the radio must switch to RX2 within that time if it does
    /// not receive a message on RX1.
    const RX_TIMEOUT: Duration = Duration::from_millis(500);

    /// How often the radio will check whether a message has been sent or received completely.
    const INTERVAL: Duration = Duration::from_millis(100);

    /// How much earlier to start listening for a message than `RX1_DELAY` and `RX2_DELAY`.
    const DELAY_MARGIN: Duration = Duration::from_micros(15);

    type Error: Debug;

    /// Basic LoRaWAN transmit. It transmits `tx`, then waits for a response on RX1, and if it does
    /// not receive anything, it waits for a response on RX2. The response is stored in `rx`. If no
    /// response is received, this method returns a timeout error.
    fn lorawan_transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        delay_1: Duration,
        delay_2: Duration,
        rate: &DataRate<R>,
    ) -> Result<Option<(usize, LoRaInfo)>, RadioError<Self::Error>>;

    /// Attempts to transmit a message.
    fn transmit(&mut self, data: &[u8]) -> Result<(), RadioError<Self::Error>>;

    /// Attempts to receive a message. This returns within one second if no message is being
    /// received, giving enough time to switch to RX2 if necessary.
    fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), RadioError<Self::Error>>;

    fn random_u8(&mut self) -> Result<u8, RadioError<Self::Error>>;

    fn random_u16(&mut self) -> Result<u16, RadioError<Self::Error>>;
}

impl<T, I, C, E> LoRaRadio for T
    where T: Transmit<Error=E>,
          T: Receive<Error=E, Info=I>,
          T: Channel<Channel=C, Error=E>,
          T: Busy<Error=E>,
          T: DelayUs<u32>,
          T: RngCore,
          I: Into<LoRaInfo>,
          C: From<LoRaChannel>,
          E: Debug
{
    type Error = E;

    fn lorawan_transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        delay_1: Duration,
        delay_2: Duration,
        rate: &DataRate<R>,
    ) -> Result<Option<(usize, LoRaInfo)>, RadioError<Self::Error>> {
        #[cfg(feature = "defmt")]
        defmt::trace!("transmitting LoRaWAN packet");
        let noise = self.random_u8()? as usize;
        self.set_channel(&rate.tx(noise).into())?;
        self.transmit(tx)?;

        #[cfg(feature = "defmt")]
        defmt::trace!("waiting for RX1 window");
        let noise = self.random_u8()? as usize;
        self.set_channel(&rate.rx1(noise).into())?;
        self.delay_us((delay_1 - Self::DELAY_MARGIN).as_micros() as u32);

        #[cfg(feature = "defmt")]
        defmt::trace!("receiving on RX1");
        match self.receive(rx) {
            Ok((n, info)) => Ok(Some((n, info))),
            Err(RadioError::Timeout) => {
                #[cfg(feature = "defmt")]
                defmt::trace!("nothing received, waiting for RX2 window");
                let noise = self.random_u8()? as usize;
                self.set_channel(&rate.rx2(noise).into())?;
                self.delay_us((delay_2 - delay_1 - Self::RX_TIMEOUT).as_micros() as u32);

                #[cfg(feature = "defmt")]
                defmt::trace!("receiving on RX2");
                match self.receive(rx) {
                    Ok((n, info)) => {
                        #[cfg(feature = "defmt")]
                        defmt::trace!("response received");
                        Ok(Some((n, info)))
                    }
                    Err(RadioError::Timeout) => {
                        #[cfg(feature = "defmt")]
                        defmt::trace!("no response");
                        Ok(None)
                    }
                    Err(error) => Err(error)
                }
            }
            Err(error) => Err(error)
        }
    }

    fn transmit(&mut self, data: &[u8]) -> Result<(), RadioError<E>> {
        self.start_transmit(data)?;

        let mut time = 0;
        loop {
            self.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.check_transmit()? {
                return Ok(());
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::TX_TIMEOUT.as_micros() {
                return Err(RadioError::Timeout);
            }
        }
    }

    fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), RadioError<E>> {
        self.start_receive()?;

        let mut time = 0;
        loop {
            self.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.check_receive(false)? {
                let (n, i) = self.get_received(buf)?;
                return Ok((n, i.into()));
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::RX_TIMEOUT.as_micros() && !self.is_busy()? {
                return Err(RadioError::Timeout);
            }
        }
    }

    fn random_u8(&mut self) -> Result<u8, RadioError<Self::Error>> {
        let mut byte = [0];
        self.try_fill_bytes(&mut byte).map_err(RadioError::Random)?;
        Ok(byte[0])
    }

    fn random_u16(&mut self) -> Result<u16, RadioError<Self::Error>> {
        let mut byte = [0, 0];
        self.try_fill_bytes(&mut byte).map_err(RadioError::Random)?;
        Ok(u16::from_le_bytes(byte))
    }
}

#[derive(Debug)]
pub enum RadioError<E> {
    Inner(E),
    Random(rand_core::Error),
    Timeout,
}

impl<E> From<E> for RadioError<E> {
    fn from(e: E) -> Self {
        RadioError::Inner(e)
    }
}
