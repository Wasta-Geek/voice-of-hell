
import type { LocalKeycode } from "./LocalKeycode";
import type { SoundEffect } from "./SoundEffect";

export type KeymapConfig = { keymap_effect_config: { [key in LocalKeycode]?: SoundEffect }, };
