use core::fmt::Debug;
use core::marker::PhantomData;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::modulation::lora::LoRaChannel;
use radio::{BasicInfo, Busy, Channel, Receive, ReceiveInfo, Transmit};
use rand_core::RngCore;

use crate::lorawan::{Settings, NEXT_DELAY};
pub use crate::radio::rate::*;
pub use crate::radio::region::*;

mod rate;
mod region;

/// Combines all the traits necessary for LoRa into one struct, and provides useful methods to
/// transmit messages.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LoRaRadio<RXTX, TIM, RNG, ERR> {
    radio: RXTX,
    tim: TIM,
    rng: RNG,
    err: PhantomData<ERR>,
}

impl<RXTX, TIM, RNG, ERR> LoRaRadio<RXTX, TIM, RNG, ERR> {
    pub fn as_radio(&self) -> &RXTX {
        &self.radio
    }

    pub fn as_mut_radio(&mut self) -> &mut RXTX {
        &mut self.radio
    }

    pub fn as_tim(&self) -> &TIM {
        &self.tim
    }

    pub fn as_mut_tim(&mut self) -> &mut TIM {
        &mut self.tim
    }

    pub fn as_rng(&self) -> &RNG {
        &self.rng
    }

    pub fn as_mut_rng(&mut self) -> &mut RNG {
        &mut self.rng
    }
}

impl<RXTX, TIM, RNG, ERR, INFO, CH> LoRaRadio<RXTX, TIM, RNG, ERR>
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
    const DELAY_MARGIN: Duration = Duration::from_micros(20);

    /// Constructs a new LoRa radio.
    pub fn new(radio: RXTX, tim: TIM, rng: RNG) -> Self {
        LoRaRadio {
            radio,
            tim,
            rng,
            err: PhantomData,
        }
    }

    pub fn lorawan_transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        tx_dr: usize,
        settings: &Settings<R>,
    ) -> Result<Option<(usize, LoRaInfo)>, RadioError<ERR>> {
        self.lorawan_transmit_delayed(tx, rx, tx_dr, settings.rx_delay(), settings)
    }

    /// Basic LoRaWAN transmit. It transmits `tx`, then waits for a response on RX1, and if it does
    /// not receive anything, it waits for a response on RX2. The response is stored in `rx`. If no
    /// response is received, this method returns a timeout error.
    pub fn lorawan_transmit_delayed<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        tx_dr: usize,
        delay: Duration,
        settings: &Settings<R>,
    ) -> Result<Option<(usize, LoRaInfo)>, RadioError<ERR>> {
        let rx1_dr = tx_dr + settings.rx1_dr_offset();
        let rx2_dr = settings.rx2_dr();

        #[cfg(feature = "defmt")]
        defmt::trace!("transmitting LoRaWAN packet");
        let noise = self.random_u8()? as usize;
        self.radio
            .set_channel(&R::get_data_rate(tx_dr)?.tx(noise).into())?;
        self.transmit_raw(tx)?;

        #[cfg(feature = "defmt")]
        defmt::trace!("waiting for RX1 window");
        self.radio
            .set_channel(&R::get_data_rate(rx1_dr)?.rx1(noise).into())?;
        self.tim
            .delay_us((delay - Self::DELAY_MARGIN).as_micros() as u32);

        #[cfg(feature = "defmt")]
        defmt::trace!("receiving on RX1");
        match self.receive_raw(rx) {
            Ok((n, info)) => Ok(Some((n, info))),
            Err(RadioError::Timeout) => {
                #[cfg(feature = "defmt")]
                defmt::trace!("nothing received, waiting for RX2 window");
                self.radio
                    .set_channel(&R::get_data_rate(rx2_dr)?.rx2().into())?;
                self.tim
                    .delay_us((NEXT_DELAY - Self::RX_TIMEOUT).as_micros() as u32);

                #[cfg(feature = "defmt")]
                defmt::trace!("receiving on RX2");
                match self.receive_raw(rx) {
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
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Attempts to transmit a message.
    fn transmit_raw(&mut self, data: &[u8]) -> Result<(), RadioError<ERR>> {
        self.radio.start_transmit(data)?;

        for _ in 0..Self::TX_TIMEOUT.as_millis() / Self::INTERVAL.as_millis() {
            self.tim.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.radio.check_transmit()? {
                return Ok(());
            }
        }

        Err(RadioError::Timeout)
    }

    /// Attempts to receive a message. This returns within one second if no message is being
    /// received, giving enough time to switch to RX2 if necessary.
    fn receive_raw(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), RadioError<ERR>> {
        self.radio.start_receive()?;

        let mut time = 0;
        loop {
            self.tim.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.radio.check_receive(false)? {
                let (n, i) = self.radio.get_received(buf)?;
                return Ok((n, i.into()));
            }

            time += Self::INTERVAL.as_micros();
            if time >= Self::RX_TIMEOUT.as_micros() && !self.radio.is_busy()? {
                return Err(RadioError::Timeout);
            }
        }
    }

    fn random_u8(&mut self) -> Result<u8, RadioError<ERR>> {
        let mut byte = [0];
        self.rng
            .try_fill_bytes(&mut byte)
            .map_err(RadioError::Random)?;
        Ok(byte[0])
    }

    pub(crate) fn random_nonce(&mut self) -> Result<u16, RadioError<ERR>> {
        let mut byte = [0, 0];
        self.rng
            .try_fill_bytes(&mut byte)
            .map_err(RadioError::Random)?;
        Ok(u16::from_le_bytes(byte))
    }
}

#[derive(Debug)]
pub enum RadioError<ERR> {
    /// The radio returned its own error.
    Radio(ERR),
    /// Failed to generate a random number.
    Random(rand_core::Error),
    UnsupportedDataRate,
    Timeout,
}

impl<ERR> From<ERR> for RadioError<ERR> {
    fn from(e: ERR) -> Self {
        RadioError::Radio(e)
    }
}

// TODO: Move to radio-hal
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
