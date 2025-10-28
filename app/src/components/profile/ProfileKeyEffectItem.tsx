import { Group, Select, SelectProps, Stack } from "@mantine/core";
import { useCallback } from "react";
import { IconFileMusic, IconFileNeutralFilled, IconVolume, IconVolume2, IconVolume3 } from "@tabler/icons-react";

import { RustSoundEffect, SoundEffectType } from "../../types";
import { useGetConfig, useUpdateConfig } from "../../hooks";
import { generateSoundEffect } from "../../types/SoundEffect";
import KeyEffectVolumeSlider from "./key_effect/VolumeSlider";
import KeyEffectFileSelector from "./key_effect/FileSelector";

type ProfileKeyEffectItemProps = {
    profileIndex: number,
    keybindIndex: number
};

function ProfileKeyEffectItem({ profileIndex, keybindIndex }: ProfileKeyEffectItemProps) {
    const { data: config } = useGetConfig();
    const updateConfig = useUpdateConfig();
    const soundEffectTypeList = Object.entries(SoundEffectType).map(([value, label]) => ({ label: label, value: value }));

    if (!config) return null;

    const handleSelectSoundEffectType = useCallback((value: string | null) => {
        // Check if config is defined
        if (config) {
            config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect = generateSoundEffect(value as RustSoundEffect);
            updateConfig.mutateAsync(config);
        }
    }, [config, profileIndex, keybindIndex]);

    const iconProps = {
        stroke: 1.5,
        color: 'currentColor',
        opacity: 0.6,
        size: 18,
    };

    const icons: Record<RustSoundEffect, React.ReactNode> = {
        DoNothing: <IconFileNeutralFilled {...iconProps} />,
        PlaySound: <IconFileMusic {...iconProps} />,
        DecreaseVolume: <IconVolume2 {...iconProps} />,
        IncreaseVolume: <IconVolume {...iconProps} />,
        ClearAllEffects: <IconVolume3 {...iconProps} />,
    };

    const renderSelectOption: SelectProps['renderOption'] = ({ option }) => (
        <Group gap="xs">
            {icons[option.value as RustSoundEffect]}
            {option.label}
        </Group>
    );

    function KeyEffectSubElements({ profileIndex, keybindIndex }: { profileIndex: number, keybindIndex: number }) {
        switch (config?.profiles[profileIndex].keybind_config[keybindIndex].sound_effect.type) {
            case RustSoundEffect.IncreaseVolume:
            case RustSoundEffect.DecreaseVolume:
                return <KeyEffectVolumeSlider profileIndex={profileIndex} keybindIndex={keybindIndex} />;
            case RustSoundEffect.PlaySound:
                return <KeyEffectFileSelector profileIndex={profileIndex} keybindIndex={keybindIndex} />;
            default:
                return null
        }
    }

    return (
        <Stack bg="gray">
            <Select
                label="Select an effect"
                data={soundEffectTypeList}
                value={config.profiles[profileIndex].keybind_config[keybindIndex].sound_effect.type}
                onChange={handleSelectSoundEffectType}
                size="md"
                renderOption={renderSelectOption}
            />
            <KeyEffectSubElements profileIndex={profileIndex} keybindIndex={keybindIndex} />
        </Stack>
    );
}

export default ProfileKeyEffectItem;