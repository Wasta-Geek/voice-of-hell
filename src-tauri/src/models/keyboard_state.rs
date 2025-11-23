use serde::{Deserialize, Serialize};

use crate::models::keycode::LocalKeycode;

/// Represents keyboars state
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardState {
    /// Pressed key(s)
    #[serde(rename = "keyPressedList")]
    pub key_pressed_list: Vec<LocalKeycode>,
    /// Released key(s)
    #[serde(rename = "keyReleasedList")]
    pub key_released_list: Vec<LocalKeycode>,
}
