use crate::lorawan::PacketError;
use crate::radio::RadioError;

#[derive(Debug)]
pub enum DeviceError<E> {
    Inner(E),
    Radio(RadioError<E>),
    Packet(PacketError<E>),
}

impl<E> From<E> for DeviceError<E> {
    fn from(e: E) -> Self {
        DeviceError::Inner(e)
    }
}

impl<E> From<RadioError<E>> for DeviceError<E> {
    fn from(e: RadioError<E>) -> Self {
        DeviceError::Radio(e)
    }
}

impl<E> From<PacketError<E>> for DeviceError<E> {
    fn from(e: PacketError<E>) -> Self {
        DeviceError::Packet(e)
    }
}
