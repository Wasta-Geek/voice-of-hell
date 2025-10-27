import { useCallback } from 'react';
import { Stack } from '@mantine/core';
import { AddArea } from '..';
import { IconMusicPlus } from '@tabler/icons-react';

import { useUpdateConfig } from '../../hooks';
import { AppConfig, RustSoundEffect } from '../../types';
import ProfileKeyEffectItem from './ProfileKeyEffectItem';

function ProfileKeyEffectList({ config }: { config: AppConfig | undefined }) {
    const updateConfig = useUpdateConfig();

    const handleAddKeybindEffect = useCallback(() => {
        // Check if config is defined
        if (config && config.last_profile_index_used) {
            const profileIndex = parseInt(config.last_profile_index_used);

            // Add new empty keybind
            config.profiles[profileIndex].keybind_config.push({
                keycode_list: [],
                sound_effect: { type: RustSoundEffect.DoNothing },
            });

            updateConfig.mutateAsync(config);
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
    const profileUsed = config.profiles[profileIndex];

    return <Stack>
        <AddArea text='Add new keybind effect' Icon={IconMusicPlus} onClick={handleAddKeybindEffect} />
        {profileUsed.keybind_config.map((_, index) => (
            <ProfileKeyEffectItem key={index} profileIndex={profileIndex} keybindIndex={index} />
        ))}
    </Stack>
}

export default ProfileKeyEffectList;