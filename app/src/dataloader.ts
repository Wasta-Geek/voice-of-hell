import { listen } from '@tauri-apps/api/event';

type AudioDevice = {
    name: string;
  };

listen<AudioDevice>('input_devices_loaded', (event) => {
    let input_device_select = document.getElementById ( "input-devices" );
    // input_device_select.
    event.map( (device, index) => {
        let option = document.createElement ( "option" );
        option.value = index;
        option.innerHTML = device.name;
        input_device_select.append ( option );
    } )
  }
);