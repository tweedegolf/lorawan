use core::fmt::Debug;

pub use crate::device::class_a::*;
use crate::device::error::DeviceError;
pub use crate::device::state::*;
use crate::lorawan::{DevNonce, JOIN_ACCEPT_DELAY1, JOIN_ACCEPT_DELAY2, JoinAccept, JoinRequest, MAX_PACKET_SIZE};
use crate::radio::{DataRate, LoRaRadio, Region};

mod class_a;
pub mod error;
mod state;

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [DeviceState] for devices that have.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Device<T, S> {
    radio: T,
    state: S,
}

impl<T, E> Device<T, Credentials>
    where T: LoRaRadio<Error=E>,
          E: Debug
{
    /// Creates a new LoRaWAN device through Over-The-Air-Activation. It must join a network with
    /// [join] before it can be used. Alternatively, an ABP-joined device can be constructed with
    /// [new_abp].
    pub fn new_otaa(radio: T, credentials: Credentials) -> Self {
        Device {
            radio,
            state: credentials,
        }
    }

    /// Attempts to join this device to a network.
    pub fn join<R: Region>(mut self) -> Result<Device<T, DeviceState<R>>, DeviceError<T, E>> {
        let dev_nonce = DevNonce::new(37);

        let join_request = JoinRequest::new(&self.state, &dev_nonce);
        let mut buf = [0; MAX_PACKET_SIZE];
        let dr0: DataRate<R> = DataRate::default();

        match self.radio.lorawan_transmit(
            join_request.payload(),
            &mut buf,
            JOIN_ACCEPT_DELAY1,
            JOIN_ACCEPT_DELAY2,
            &dr0,
        )? {
            None => Err(DeviceError::Join(self)),
            Some((n, _)) => {
                // TODO: Manage state
                let (device_state, _) = JoinAccept::from_data(&mut buf[..n])?
                    .extract_state::<R>(&self.state, &dev_nonce);

                let device = Device {
                    radio: self.radio,
                    state: device_state,
                };

                Ok(device)
            }
        }


    }
}

impl<T, R, E> Device<T, DeviceState<R>>
    where T: LoRaRadio<Error=E>,
          R: Region,
          E: Debug
{
    /// Creates a joined device through Activation By Personalization. Consider using [new_otaa]
    /// instead, as it is more secure.
    pub fn new_abp(radio: T, session: Session) -> Self {
        let state = DeviceState::new(session);

        Device {
            radio,
            state,
        }
    }

    /// Configures this device to have class A behavior: listening for downlinks only after
    /// transmitting an uplink.
    pub fn into_class_a(self) -> ClassA<T, R> {
        self.into()
    }
}
