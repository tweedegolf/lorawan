use core::time::Duration;

pub const RECEIVE_DELAY: Duration = Duration::from_secs(1);
pub const JOIN_ACCEPT_DELAY: Duration = Duration::from_secs(5);
pub const NEXT_DELAY: Duration = Duration::from_secs(1);

pub const ADR_ACK_LIMIT: usize = 64;
pub const ADR_ACK_DELAY: usize = 32;
