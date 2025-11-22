import { useCallback } from "react";
import { Grid } from "@mantine/core";
import { IconMusicPlus } from "@tabler/icons-react";

import { useConfig } from "@/hooks";
import { RustSoundEffect } from "@/types";
import ProfileKeyEffectItem from "@/components/profile/ProfileKeyEffectItem";
import { ButtonWithIcon } from "@/components";

function ProfileKeyEffectList() {
  const [config, setConfig] = useConfig();

  const handleAddKeybindEffect = useCallback(() => {
    // Check if config is defined
    if (config && config.last_profile_index_used) {
      const profileIndex = parseInt(config.last_profile_index_used);

      const new_config = { ...config };
      // Add new empty keybind
      new_config.profiles[profileIndex].keybind_config.push({
        keycode_list: [],
        sound_effect: { type: RustSoundEffect.DoNothing },
      });

      setConfig(new_config);
    }
  }, [config, setConfig]);

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

  return (
    <>
      <ButtonWithIcon
        text="Add new keybind effect"
        key={config.last_profile_index_used}
        Icon={IconMusicPlus}
        onClick={handleAddKeybindEffect}
      />
      <Grid gutter="xs">
        {config.profiles[profileIndex].keybind_config.map((_, index) => (
          <Grid.Col span="auto" key={index}>
            <ProfileKeyEffectItem
              profileIndex={profileIndex}
              keybindIndex={index}
            />
          </Grid.Col>
        ))}
      </Grid>
    </>
  );
}

export default ProfileKeyEffectList;
