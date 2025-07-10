import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import "./App.css";

type AudioDevice = {
  name: string;
};

function Device() {
  const [inputDeviceList, setInputDeviceList] = useState<AudioDevice[]>([]);
  const [outputDeviceList, setOutputDeviceList] = useState<AudioDevice[]>([]);

  async function inputDeviceOnChange(event: any) {
    if (event.target.value != "") {
      invoke('input_device_selected', {deviceName : event.target.value});  
    }
  }

  async function outputDeviceOnChange(event: any) {
    if (event.target.value != "") {
      invoke('output_device_selected', {deviceName : event.target.value});  
    }
  }

  useEffect(() => {
    // TODO I guess a rust command to be invoked would be more clean but fine for now
    listen<AudioDevice[]>('input_devices_refreshed', (event) => {
      setInputDeviceList(event.payload);
    } // Wait for listen to be ready to call Rust
    ).then((unlisten) => {
      // Call Rust command telling frontend is ready
      invoke('app_ready', {});
    });

    listen<AudioDevice[]>('output_devices_refreshed', (event) => {
      setOutputDeviceList(event.payload);
    } // Wait for listen to be ready to call Rust
    ).then((unlisten) => {
      // Call Rust command telling frontend is ready
      invoke('app_ready', {});
    });
  }, [])

  return (
    <main className="container">
      <div>
        <h1>Input device used:</h1>

        <select onChange={inputDeviceOnChange}>
          <option></option>
          {
            inputDeviceList.map((device) => <option>{device.name}</option>)
          }
        </select>

        <h1>Output device used:</h1>
        <select onChange={outputDeviceOnChange}>
          <option></option>
          {
            outputDeviceList.map((device) => <option>{device.name}</option>)
          }
        </select>
      </div>
    </main>
  );
}

export default Device;