import { ChangeEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Stack } from "@mantine/core";
import { useQueryClient } from "@tanstack/react-query";

import { useConfig } from "@/hooks";
import { AppConfig } from "@/types/AppConfig";

type AudioDevice = {
  name: string;
};

type DeviceSelectProps = {
  deviceList: AudioDevice[];
  selectedDeviceName: string | null;
  onChange: (event: ChangeEvent<HTMLSelectElement>) => void;
};

/**
 * Device selection, the stored device is kept visible even if not currently available (unplugged...)
 * @param {DeviceSelectProps} props component prop
 */
function DeviceSelect({
  deviceList,
  selectedDeviceName,
  onChange,
}: DeviceSelectProps) {
  const isSelectedDeviceMissing =
    selectedDeviceName != null &&
    !deviceList.some((device) => device.name == selectedDeviceName);

  return (
    <select value={selectedDeviceName ?? ""} onChange={onChange}>
      <option></option>
      {isSelectedDeviceMissing && (
        <option value={selectedDeviceName}>
          {selectedDeviceName} (unavailable)
        </option>
      )}
      {deviceList.map((device, index) => (
        <option key={index}>{device.name}</option>
      ))}
    </select>
  );
}

/**
 * Device tab
 */
function Devices() {
  // TODO Move into an external component ?
  const [inputDeviceList, setInputDeviceList] = useState<AudioDevice[]>([]);
  const [outputDeviceList, setOutputDeviceList] = useState<AudioDevice[]>([]);
  const [config] = useConfig();
  const queryClient = useQueryClient();

  /**
   * Use the given device, then refresh config (Rust stores the device as last used)
   * @param command Rust command to invoke
   * @param deviceName device to use
   */
  async function selectDevice(command: string, deviceName: string) {
    const newConfig = await invoke<AppConfig>(command, { deviceName });
    queryClient.setQueryData(["config"], newConfig);
  }

  /**
   * Handler called when input device selected changed
   * @param event Html component
   */
  async function handleInputDeviceOnChange(
    event: ChangeEvent<HTMLSelectElement>,
  ) {
    if (event.target.value != "") {
      selectDevice("input_device_selected", event.target.value);
    }
  }

  /**
   * Handler called when output device selected changed
   * @param event Html component
   */
  async function handleOutputDeviceOnChange(
    event: ChangeEvent<HTMLSelectElement>,
  ) {
    if (event.target.value != "") {
      selectDevice("output_device_selected", event.target.value);
    }
  }

  useEffect(() => {
    // TODO I guess a rust command to be invoked would be more clean but fine for now
    listen<AudioDevice[]>(
      "input_devices_refreshed",
      (event) => {
        setInputDeviceList(event.payload);
      }, // Wait for listen to be ready to call Rust
    ).then(() => {
      // Call Rust command telling frontend is ready
      invoke("app_ready", {});
    });

    listen<AudioDevice[]>(
      "output_devices_refreshed",
      (event) => {
        setOutputDeviceList(event.payload);
      }, // Wait for listen to be ready to call Rust
    ).then(() => {
      // Call Rust command telling frontend is ready
      invoke("app_ready", {});
    });
  }, []);

  return (
    <Stack align="center" gap="xs">
      <h1>Input device used:</h1>
      <DeviceSelect
        deviceList={inputDeviceList}
        selectedDeviceName={config?.last_input_device_used ?? null}
        onChange={handleInputDeviceOnChange}
      />

      <h1>Output device used:</h1>
      <DeviceSelect
        deviceList={outputDeviceList}
        selectedDeviceName={config?.last_output_device_used ?? null}
        onChange={handleOutputDeviceOnChange}
      />
    </Stack>
  );
}

export default Devices;
