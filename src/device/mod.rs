use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::modulation::lora::LoRaChannel;
use radio::{Busy, Channel, Receive, Transmit};
use rand_core::RngCore;

pub use crate::device::class_a::*;
use crate::device::error::DeviceError;
pub use crate::device::state::*;
use crate::lorawan::{
    DevNonce, JoinAccept, JoinRequest, Settings, JOIN_ACCEPT_DELAY, MAX_PACKET_SIZE,
};
use crate::radio::{LoRaInfo, LoRaRadio, Region};

mod class_a;
pub mod error;
mod state;

type JoinResult<RXTX, TIM, RNG, ERR, R> =
    Result<Device<RXTX, TIM, RNG, ERR, DeviceState<R>>, DeviceError<RXTX, TIM, RNG, ERR>>;

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [DeviceState] for devices that have.
///
/// [Credentials]: crate::device::Credentials
/// [DeviceState]: crate::device::DeviceState
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Device<RXTX, TIM, RNG, ERR, STATE> {
    radio: LoRaRadio<RXTX, TIM, RNG, ERR>,
    state: STATE,
}

impl<RXTX, TIM, RNG, ERR, STATE> Device<RXTX, TIM, RNG, ERR, STATE> {
    pub fn as_lora_radio(&self) -> &LoRaRadio<RXTX, TIM, RNG, ERR> {
        &self.radio
    }

    pub fn as_mut_lora_radio(&mut self) -> &mut LoRaRadio<RXTX, TIM, RNG, ERR> {
        &mut self.radio
    }
}

impl<RXTX, TIM, RNG, ERR, INFO, CH> Device<RXTX, TIM, RNG, ERR, Credentials>
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
    /// Creates a new LoRaWAN device through Over-The-Air-Activation. It must join a network with
    /// [join] before it can be used. Alternatively, an ABP-joined device can be constructed with
    /// [new_abp].
    pub fn new_otaa(radio: LoRaRadio<RXTX, TIM, RNG, ERR>, credentials: Credentials) -> Self {
        Device {
            radio,
            state: credentials,
        }
    }

    /// Attempts to join this device to a network.
    pub fn join<R: Region>(mut self) -> JoinResult<RXTX, TIM, RNG, ERR, R> {
        let dev_nonce = DevNonce::new(self.radio.random_nonce()?);

        let join_request = JoinRequest::new(&self.state, &dev_nonce);
        let mut buf = [0; MAX_PACKET_SIZE];

        match self.radio.lorawan_transmit_delayed::<R>(
            join_request.payload(),
            &mut buf,
            0,
            JOIN_ACCEPT_DELAY,
            &Settings::default(),
        )? {
            None => Err(DeviceError::Join(self)),
            Some((n, _)) => {
                let state =
                    JoinAccept::from_data(&mut buf[..n])?.extract_state(&self.state, &dev_nonce);

                let device = Device {
                    radio: self.radio,
                    state,
                };

                Ok(device)
            }
        }
    }
}

impl<RXTX, TIM, RNG, ERR, INFO, CH, R> Device<RXTX, TIM, RNG, ERR, DeviceState<R>>
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
    /// Creates a joined device through Activation By Personalization. Consider using [new_otaa]
    /// instead, as it is more secure.
    pub fn new_abp(radio: LoRaRadio<RXTX, TIM, RNG, ERR>, session: Session) -> Self {
        let state = DeviceState::new(session, Settings::default());

        Device { radio, state }
    }

    /// Configures this device to have class A behavior: listening for downlinks only after
    /// transmitting an uplink.
    pub fn into_class_a(self) -> ClassA<RXTX, TIM, RNG, ERR, R> {
        self.into()
    }
}
