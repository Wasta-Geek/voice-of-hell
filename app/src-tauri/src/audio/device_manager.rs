use std::cmp;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, SampleFormat, SampleRate, StreamConfig};

use crate::audio::globals::{BUFFER_SIZE, PREFERRED_SAMPLE_RATE};
use crate::audio::stream_manager::StreamManager;
use crate::error::MyError;
use crate::models::common::Device as CommonDevice;

/// Manages audio devices
pub struct DeviceManager {
    /// List of available input device name list
    pub input_devices: Vec<CommonDevice>,
    /// List of available output device name list
    pub output_devices: Vec<CommonDevice>,
    /// List of available input devices
    internal_input_devices: Vec<Device>,
    /// List of available output devices
    internal_output_devices: Vec<Device>,
    /// Internal cpal host
    host: Host,
    /// Manage streams
    stream_manager: StreamManager,
    /// Current input device
    input_device_selected: Option<Device>,
    /// Current output device
    output_device_selected: Option<Device>,
}

impl DeviceManager {
    /// Generates new DeviceManager instance
    pub fn new() -> Self {
        let mut instance = DeviceManager {
            host: cpal::default_host(),
            internal_input_devices: Vec::new(),
            internal_output_devices: Vec::new(),
            input_devices: Vec::new(),
            output_devices: Vec::new(),
            stream_manager: StreamManager::new(),
            input_device_selected: None,
            output_device_selected: None,
        };

        // Refresh devices immediately (ready to uses)
        instance.refresh_devices();

        instance
    }

    /// Refresh devices list
    pub fn refresh_devices(&mut self) {
        // Assign input devices
        self.internal_input_devices = match self.host.input_devices() {
            Ok(iter) => iter.collect(),
            Err(err) => {
                log::warn!("Couldn't find input devices, details: {err}");
                Vec::<Device>::new()
            }
        };

        self.input_devices = self
            .internal_input_devices
            .iter()
            .map(|device| CommonDevice {
                name: device.name().expect("couldn't retrieves device name"),
            })
            .collect();

        // Assign output devices
        self.internal_output_devices = match self.host.output_devices() {
            Ok(iter) => iter.collect(),
            Err(err) => {
                log::warn!("Couldn't find output devices, details: {err}");
                Vec::<Device>::new()
            }
        };

        self.output_devices = self
            .internal_output_devices
            .iter()
            .map(|device| CommonDevice {
                name: device.name().expect("couldn't retrieves device name"),
            })
            .collect();
    }

    /// Set a target device as input
    pub fn set_input_device(&mut self, target_device_name: &str) -> Result<(), MyError> {
        let input_device = self
            .find_device_from_name(target_device_name, &self.internal_input_devices)
            .map_err(|_| MyError {})?;
        self.input_device_selected = Some(input_device.clone());

        match self.input_device_selected.is_some() && self.output_device_selected.is_some() {
            true => self.start_streams(),
            false => Err(MyError {}),
        }
    }

    /// Set a target device as output
    pub fn set_output_device(&mut self, target_device_name: &str) -> Result<(), MyError> {
        let output_device = self
            .find_device_from_name(target_device_name, &self.internal_output_devices)
            .map_err(|_| MyError {})?;
        self.output_device_selected = Some(output_device.clone());

        match self.input_device_selected.is_some() && self.output_device_selected.is_some() {
            true => self.start_streams(),
            false => Err(MyError {}),
        }
    }

    /// Start input / output streams
    fn start_streams(&mut self) -> Result<(), MyError> {
        let input_device = self.input_device_selected.as_ref().ok_or(MyError {})?;
        let output_device = self.output_device_selected.as_ref().ok_or(MyError {})?;

        let sample_rate = self.find_compatible_sample_rate();
        let config = StreamConfig {
            channels: 2,
            sample_rate: sample_rate,
            // Note: Fixed buffersize seems bugged and ignored on WAsio
            buffer_size: cpal::BufferSize::Fixed(BUFFER_SIZE),
        };

        self.stream_manager
            .start_streams(input_device, output_device, &config)?;

        return Ok(());
    }

    /// [temporary] Find a suitable sample rate
    fn find_compatible_sample_rate(&self) -> SampleRate {
        let available_input_configs = self
            .input_device_selected
            .as_ref()
            .unwrap()
            .supported_input_configs()
            .expect("Couldn't get supported configs");
        let available_output_configs = self
            .output_device_selected
            .as_ref()
            .unwrap()
            .supported_output_configs()
            .expect("Couldn't get supported configs");

        let supported_input_config = available_input_configs
            .into_iter()
            .find(|config| config.sample_format() == SampleFormat::F32)
            .unwrap();
        let supported_output_config = available_output_configs
            .into_iter()
            .find(|config| config.sample_format() == SampleFormat::F32)
            .unwrap();

        let highest_min_sample_rate = cmp::max(
            supported_input_config.min_sample_rate(),
            supported_output_config.min_sample_rate(),
        );
        let lowest_max_sample_rate = cmp::min(
            supported_input_config.max_sample_rate(),
            supported_output_config.max_sample_rate(),
        );

        // TODO Need to deal with resampling because devices won't always match sample rate
        // Check if prefered sample rate is available for both device
        match SampleRate(PREFERRED_SAMPLE_RATE) >= highest_min_sample_rate
            && SampleRate(PREFERRED_SAMPLE_RATE) <= lowest_max_sample_rate
        {
            true => SampleRate(PREFERRED_SAMPLE_RATE),
            // Default to min sample rate for both
            false => lowest_max_sample_rate,
        }
    }

    fn find_device_from_name(
        &self,
        target_device_name: &str,
        device_list: &Vec<Device>,
    ) -> Result<Device, ()> {
        for device in device_list {
            let device_name = device.name();
            if device_name.is_err() {
                continue;
            };
            if device_name.unwrap() == target_device_name {
                return Ok(device.clone());
            }
        }
        return Err(());
    }
}
