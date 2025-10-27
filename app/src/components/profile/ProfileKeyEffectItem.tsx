import { Group, Select, SelectProps, Stack } from "@mantine/core";
import { useCallback } from "react";
import { IconFileMusic } from "@tabler/icons-react";

import { RustSoundEffect, SoundEffectType } from "../../types";
import { useGetConfig, useUpdateConfig } from "../../hooks";
import { generateSoundEffect } from "../../types/SoundEffect";
import KeyEffectVolumeSlider from "./key_effect/VolumeSlider";

type ProfileKeyEffectItemProps = {
    profileIndex: number,
    keybindIndex: number
};

function ProfileKeyEffectItem({ profileIndex, keybindIndex }: ProfileKeyEffectItemProps) {
    const { data: config } = useGetConfig();
    const updateConfig = useUpdateConfig();
    const soundEffectTypeList = Object.entries(SoundEffectType).map(([value, label]) => ({ label: label, value: value }));

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

    const icons: Record<string, React.ReactNode> = {
        left: <IconFileMusic {...iconProps} />,
        center: <IconFileMusic {...iconProps} />,
        right: <IconFileMusic {...iconProps} />,
        justify: <IconFileMusic {...iconProps} />,
    };

    const renderSelectOption: SelectProps['renderOption'] = ({ option, checked }) => (
        <Group flex="1" gap="xs">
            {icons[option.value]}
            {option.label}
        </Group>
    );

    return (
        <Stack bg="gray">
            <Select
                label="Select an effect"
                data={soundEffectTypeList}
                value={config?.profiles[profileIndex].keybind_config[keybindIndex].sound_effect.type}
                onChange={handleSelectSoundEffectType}
                size="md"
                renderOption={renderSelectOption}
            />
            <KeyEffectVolumeSlider soundEffect={config?.profiles[profileIndex].keybind_config[keybindIndex].sound_effect} profileIndex={profileIndex} keybindIndex={keybindIndex} />
        </Stack>
    );
}

export default ProfileKeyEffectItem;