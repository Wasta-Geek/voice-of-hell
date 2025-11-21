import { useCallback, useState } from "react";
import { Slider } from "@mantine/core";

import { RustSoundEffect, SoundEffect } from "@/types";
import { useConfig } from "@/hooks";

type KeyEffectVolumeSliderProps = {
    profileIndex: number,
    keybindIndex: number
};

function KeyEffectVolumeSlider({ profileIndex, keybindIndex }: KeyEffectVolumeSliderProps) {
    const [config, setConfig] = useConfig();

    if (!config) return null;

    const soundEffect = config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect as Extract<SoundEffect, { type: RustSoundEffect.IncreaseVolume | RustSoundEffect.DecreaseVolume }>;
    const [volume, setVolume] = useState(soundEffect?.volume ?? 50);

    const handleVolumeChanged = useCallback((volume: number) => {
        // Check if config is defined
        if (config) {
            setVolume(volume);
        }
    }, [config, profileIndex, keybindIndex]);

    const handleVolumeChangedEnd = useCallback((volume: number) => {
        // Check if config is defined
        if (config) {
            const soundEffectTyped = config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect as Extract<SoundEffect, { type: RustSoundEffect.IncreaseVolume | RustSoundEffect.DecreaseVolume }>;

            soundEffectTyped.volume = volume;
            setConfig(config);
        }
    }, [config, profileIndex, keybindIndex]);

    return (
        <Slider miw="200px" color="blue" value={volume} onChange={handleVolumeChanged} onChangeEnd={handleVolumeChangedEnd} />
    );
};

export default KeyEffectVolumeSlider;