import { useCallback } from 'react';
import { Select, Stack } from '@mantine/core';
import { IconMusicPlus } from '@tabler/icons-react';

import { useGetConfig, useUpdateConfig } from '@/hooks';
import { RustSoundEffect } from '@/types';
import ProfileKeyEffectItem from '@/components/profile/ProfileKeyEffectItem';
import { ButtonWithIcon } from '@/components';

type ProfileKeyEffectProps = {};

function ProfileKeyEffectList({ }: ProfileKeyEffectProps) {
    const { data: config } = useGetConfig();
    const updateConfig = useUpdateConfig();

    const handleAddKeybindEffect = useCallback(() => {
        // Check if config is defined
        if (config && config.last_profile_index_used) {
            const profileIndex = parseInt(config.last_profile_index_used);

            let new_config = {...config};
            // Add new empty keybind
            new_config.profiles[profileIndex].keybind_config.push({
                keycode_list: [],
                sound_effect: { type: RustSoundEffect.DoNothing },
            });

            updateConfig.mutateAsync(new_config);
        }
    }, [config]);

    // Check if a profile is currently used
    if (!config || !config?.last_profile_index_used) {
        // Nothing to show
        return null;
    }
    const profileIndex = parseInt(config.last_profile_index_used);
    // If last profile index doesn't exists anymore
    if (profileIndex >= config.profiles.length) {
        return null;
    }

    return <Stack>
        <ButtonWithIcon text='Add new keybind effect' key={config.last_profile_index_used} Icon={IconMusicPlus} onClick={handleAddKeybindEffect} />
        {config.profiles[profileIndex].keybind_config.map((_, index) => (
            <ProfileKeyEffectItem key={index} profileIndex={profileIndex} keybindIndex={index} />
        ))}
    </Stack>
}

export default ProfileKeyEffectList;