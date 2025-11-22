use serde::{Deserialize, Serialize};

use crate::models::keycode::LocalKeycode;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyboardState {
    #[serde(rename = "keyPressedList")]
    pub key_pressed_list: Vec<LocalKeycode>,
    #[serde(rename = "keyReleasedList")]
    pub key_released_list: Vec<LocalKeycode>,
}
