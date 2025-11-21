use crate::{audio::device_manager::DeviceManager, profile::config_manager::ConfigManager};

pub struct SharedState {
    pub device_manager: DeviceManager,
    pub config_manager: ConfigManager
}
