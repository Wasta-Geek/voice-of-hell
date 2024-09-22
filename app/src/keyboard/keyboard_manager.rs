use std::collections::HashMap;

use device_query::{CallbackGuard, DeviceEvents, DeviceState, Keycode};
use once_cell::sync::Lazy;

// use crate::keyboard::actions::utils::mute_all_effects;

static KEYMAP_PRESSED_ACTION: Lazy<HashMap<Keycode, fn () -> ()>> = Lazy::new(|| {
    HashMap::from ([
        // ex:
        //  (Keycode::Up, mute_all_effects as fn() -> ())
    ])
});
static KEYMAP_RELEASED_ACTION: Lazy<HashMap<Keycode, fn () -> ()>> = Lazy::new(|| {HashMap::new()});

pub struct KeyboardManager {
    device_state: DeviceState,
    key_released_guard: Option<CallbackGuard<fn(&Keycode)>>,
    key_pressed_guard: Option<CallbackGuard<fn(&Keycode)>>,
}

impl KeyboardManager {
    pub fn new() -> Self {
        let mut ret = KeyboardManager {
            device_state: DeviceState::new(),
            key_pressed_guard: None,
            key_released_guard: None,
        };
        ret.registers_key_state_changed();
        ret
    }

    fn registers_key_state_changed(&mut self ) {
        self.key_pressed_guard = Some(self.device_state.on_key_down(on_key_down));
        self.key_released_guard = Some(self.device_state.on_key_up(on_key_up));
    }
}

pub fn on_key_up ( keycode: &Keycode ) {
    if let Some(callback) = KEYMAP_RELEASED_ACTION.get(keycode) {
        callback();
    }
}

pub fn on_key_down ( keycode: &Keycode ) {
    if let Some(callback) = KEYMAP_PRESSED_ACTION.get(keycode) {
        callback();
    }
}