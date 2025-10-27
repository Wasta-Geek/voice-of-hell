import { useCallback, useState } from "react";
import { RustSoundEffect, SoundEffect } from "../../../types";
import { Slider } from "@mantine/core";
import { useGetConfig, useUpdateConfig } from "../../../hooks";

type KeyEffectVolumeSliderProps = { 
    soundEffect: SoundEffect | undefined,
    profileIndex: number,
    keybindIndex: number
 };

function KeyEffectVolumeSlider ({ soundEffect, profileIndex, keybindIndex }: KeyEffectVolumeSliderProps ) {
    const { data: config } = useGetConfig();
    const updateConfig = useUpdateConfig();

    if (!soundEffect || !([RustSoundEffect.IncreaseVolume, RustSoundEffect.DecreaseVolume].includes(soundEffect.type))) return null;

    const [volume, setVolume] = useState(soundEffect.volume);

    const handleVolumeChanged = useCallback((volume: number) => {
        // Check if config is defined
        if (config) {
            setVolume(volume);
        }
    }, [config, profileIndex, keybindIndex]);

    const handleVolumeChangedEnd = useCallback((volume: number) => {
        // Check if config is defined
        if (config) {
            config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect.volume = volume;
            updateConfig.mutateAsync(config);
        }
    }, [config, profileIndex, keybindIndex]);

    return <Slider color="blue" value={volume} onChange={handleVolumeChanged} onChangeEnd={handleVolumeChangedEnd} />
};

export default KeyEffectVolumeSlider;