import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Stack } from "@mantine/core";

type AudioDevice = {
  name: string;
};

function Devices() {
  // TODO Move into an external component ?
  const [inputDeviceList, setInputDeviceList] = useState<AudioDevice[]>([]);
  const [outputDeviceList, setOutputDeviceList] = useState<AudioDevice[]>([]);

  // Handler called when input device selected changed
  async function handleInputDeviceOnChange(event: any) {
    if (event.target.value != "") {
      invoke('input_device_selected', { deviceName: event.target.value });
    }
  }

  // Handler called when output device selected changed
  async function handleOutputDeviceOnChange(event: any) {
    if (event.target.value != "") {
      invoke('output_device_selected', { deviceName: event.target.value });
    }
  }

  useEffect(() => {
    // TODO I guess a rust command to be invoked would be more clean but fine for now
    listen<AudioDevice[]>('input_devices_refreshed', (event) => {
      setInputDeviceList(event.payload);
    } // Wait for listen to be ready to call Rust
    ).then(() => {
      // Call Rust command telling frontend is ready
      invoke('app_ready', {});
    });

    listen<AudioDevice[]>('output_devices_refreshed', (event) => {
      setOutputDeviceList(event.payload);
    } // Wait for listen to be ready to call Rust
    ).then(() => {
      // Call Rust command telling frontend is ready
      invoke('app_ready', {});
    });
  }, []);

  return (
    <Stack align="center" gap="xs">
      <h1>Input device used:</h1>

      <select onChange={handleInputDeviceOnChange}>
        <option></option>
        {
          inputDeviceList.map((device, index) => <option key={index}>{device.name}</option>)
        }
      </select>

      <h1>Output device used:</h1>
      <select onChange={handleOutputDeviceOnChange}>
        <option></option>
        {
          outputDeviceList.map((device, index) => <option key={index}>{device.name}</option>)
        }
      </select>
    </Stack>
  );
}

export default Devices;