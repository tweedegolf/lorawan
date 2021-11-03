use radio::blocking::BlockingError;
use crate::lorawan::PacketError;

#[derive(Debug)]
pub enum DeviceError<E> {
    Inner(E),
    Blocking(BlockingError<E>),
    Packet(PacketError<E>),
}

impl<E> From<E> for DeviceError<E> {
    fn from(e: E) -> Self {
        DeviceError::Inner(e)
    }
}

impl<E> From<BlockingError<E>> for DeviceError<E> {
    fn from(e: BlockingError<E>) -> Self {
        DeviceError::Blocking(e)
    }
}

impl<E> From<PacketError<E>> for DeviceError<E> {
    fn from(e: PacketError<E>) -> Self {
        DeviceError::Packet(e)
    }
}
