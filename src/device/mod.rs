use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::modulation::lora::LoRaChannel;
use radio::{Busy, Channel, Receive, Transmit};
use rand_core::RngCore;

pub use crate::device::class_a::*;
use crate::device::error::DeviceError;
pub use crate::device::state::*;
use crate::lorawan::{
    DevNonce, JoinAccept, JoinRequest, Settings, DELTA_RECEIVE_DELAY, JOIN_ACCEPT_DELAY1,
    JOIN_ACCEPT_DELAY2, MAX_PACKET_SIZE,
};
use crate::radio::{LoRaInfo, LoRaRadio, RadioError, Region};

mod class_a;
pub mod error;
mod state;

type JoinResult<RXTX, TIM, RNG, ERR> =
    Result<Device<RXTX, TIM, RNG, ERR, DeviceState>, DeviceError<RXTX, TIM, RNG, ERR>>;

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [DeviceState] for devices that have.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Device<RXTX, TIM, RNG, ERR, STATE> {
    radio: LoRaRadio<RXTX, TIM, RNG, ERR>,
    state: STATE,
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
    pub fn join<R: Region>(mut self) -> JoinResult<RXTX, TIM, RNG, ERR> {
        let dev_nonce = DevNonce::new(self.radio.random_nonce()?);

        let join_request = JoinRequest::new(&self.state, &dev_nonce);
        let mut buf = [0; MAX_PACKET_SIZE];

        // TODO: Refactor
        match self.radio.lorawan_transmit::<R>(
            join_request.payload(),
            &mut buf,
            JOIN_ACCEPT_DELAY1,
            JOIN_ACCEPT_DELAY2,
            0,
            0,
            0,
        )? {
            None => Err(DeviceError::Join(self)),
            Some((n, _)) => {
                let device_state =
                    JoinAccept::from_data(&mut buf[..n])?.extract_state(&self.state, &dev_nonce);

                let device = Device {
                    radio: self.radio,
                    state: device_state,
                };

                Ok(device)
            }
        }
    }
}

impl<RXTX, TIM, RNG, ERR, INFO, CH> Device<RXTX, TIM, RNG, ERR, DeviceState>
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
    /// Creates a joined device through Activation By Personalization. Consider using [new_otaa]
    /// instead, as it is more secure.
    pub fn new_abp(radio: LoRaRadio<RXTX, TIM, RNG, ERR>, session: Session) -> Self {
        let state = DeviceState::new(session, Settings::default());

        Device { radio, state }
    }

    /// Configures this device to have class A behavior: listening for downlinks only after
    /// transmitting an uplink.
    pub fn into_class_a(self) -> ClassA<RXTX, TIM, RNG, ERR> {
        self.into()
    }

    pub fn transmit_raw<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
    ) -> Result<Option<(usize, LoRaInfo)>, RadioError<ERR>> {
        // TODO: Refactor
        let delay = self.state.settings().rx_delay();
        let tx_dr = self.state.data_rate();
        let rx1_dr_offset = self.state.settings().rx1_dr_offset();
        let rx2_dr = self.state.settings().rx2_dr();
        self.radio.lorawan_transmit::<R>(
            tx,
            rx,
            delay,
            delay + DELTA_RECEIVE_DELAY,
            tx_dr,
            rx1_dr_offset,
            rx2_dr,
        )
    }
}
