use crate::device::{Credentials, Device};
use crate::lorawan::PacketError;
use crate::radio::RadioError;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<T, E> {
    Inner(E),
    Join(Device<T, Credentials>),
    Radio(RadioError<E>),
    Packet(PacketError<E>),
}

impl<T, E> From<E> for DeviceError<T, E> {
    fn from(e: E) -> Self {
        DeviceError::Inner(e)
    }
}

impl<T, E> From<RadioError<E>> for DeviceError<T, E> {
    fn from(e: RadioError<E>) -> Self {
        DeviceError::Radio(e)
    }
}

impl<T, E> From<PacketError<E>> for DeviceError<T, E> {
    fn from(e: PacketError<E>) -> Self {
        DeviceError::Packet(e)
    }
}
