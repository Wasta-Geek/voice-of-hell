import { LocalKeycode } from "./LocalKeycode";
import { SoundEffect } from "./SoundEffect";

export type KeybindEffectItem = {
    keycode_list: LocalKeycode[],
    sound_effect: SoundEffect
};