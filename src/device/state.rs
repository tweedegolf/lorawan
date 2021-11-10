use crate::lorawan::{AppEui, AppKey, AppSKey, DevAddr, DevEui, NwkSKey};

/// Credentials needed to join a device to a network. A device that has not joined a network will
/// use this as state.
#[derive(Debug)]
pub struct Credentials {
    app_eui: AppEui,
    dev_eui: DevEui,
    app_key: AppKey,
}

impl Credentials {
    pub fn app_eui(&self) -> &AppEui {
        &self.app_eui
    }

    pub fn dev_eui(&self) -> &DevEui {
        &self.dev_eui
    }

    pub fn app_key(&self) -> &AppKey {
        &self.app_key
    }
}

/// Represents the state of a device that has joined a network.
#[derive(Debug)]
pub struct DeviceState {
    session: Session,
    settings: Settings,
    fcnt_up: u32,
    fcnt_down: u32,
}

impl DeviceState {
    pub fn new(session: Session, settings: Settings) -> Self {
        DeviceState {
            session,
            settings,
            fcnt_up: 0,
            fcnt_down: 0,
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn change_settings(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn fcnt_up(&self) -> u32 {
        self.fcnt_up
    }

    pub fn fcnt_down(&self) -> u32 {
        self.fcnt_down
    }

    pub fn increment_fcnt_up(&mut self) {
        self.fcnt_up += 1;
    }

    pub fn increment_fcnt_down(&mut self) {
        self.fcnt_down += 1;
    }
}

/// Session data for a device joined to a network.
#[derive(Debug)]
pub struct Session {
    dev_addr: DevAddr,
    nwk_skey: NwkSKey,
    app_skey: AppSKey,
}

impl Session {
    /// Creates a session directly for ABP.
    pub fn new(dev_addr: DevAddr, nwk_skey: NwkSKey, app_skey: AppSKey) -> Self {
        Session {
            dev_addr,
            nwk_skey,
            app_skey,
        }
    }

    pub fn dev_addr(&self) -> &DevAddr {
        &self.dev_addr
    }

    pub fn nwk_skey(&self) -> &NwkSKey {
        &self.nwk_skey
    }

    pub fn app_skey(&self) -> &AppSKey {
        &self.app_skey
    }
}

/// Custom settings that the device uses to improve communication with the network, such as
/// different delays, spreading factors, frequencies, etc. These are changed automatically using MAC
/// commands.
#[derive(Debug)]
pub struct Settings {
    rx_delay: u8,
}

impl Settings {
    pub fn set_rx_delay(&mut self, rx_delay: u8) -> &mut Self {
        self.rx_delay = rx_delay;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            rx_delay: 0
        }
    }
}
