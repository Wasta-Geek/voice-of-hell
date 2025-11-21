import { FileInput } from "@mantine/core";
import { useCallback, useState } from "react";

import { RustSoundEffect, SoundEffect } from "@/types";
import { useConfig } from "@/hooks";

type KeyEffectFileSelectorProps = {
    profileIndex: number,
    keybindIndex: number
};

function KeyEffectFileSelector({ profileIndex, keybindIndex }: KeyEffectFileSelectorProps) {
    const [config, setConfig] = useConfig();

    if (!config) return null;

    let soundEffect = config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect as Extract<SoundEffect, { type: RustSoundEffect.PlaySound }>;
    console.log(soundEffect)
    const [value, setValue] = useState<File | null>(soundEffect.name != "" ? new File([], soundEffect.name) : null);

    const handleSelectFile = useCallback((file: File | null) => {
        setValue(file);
        soundEffect.name = file?.name ?? "";
        soundEffect.lastModified = file?.lastModified ?? null;
        soundEffect.path = file?.webkitRelativePath ?? null;
        setConfig(config);
    }, []);

    return <FileInput
        placeholder="Select a sound file"
        value={value}
        onChange={handleSelectFile}
        clearable
    />;
}

export default KeyEffectFileSelector;