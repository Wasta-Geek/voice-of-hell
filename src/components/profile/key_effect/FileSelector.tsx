import { CloseButton, TextInput } from "@mantine/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useCallback } from "react";

import { RustSoundEffect, SoundEffect } from "@/types";
import { useConfig } from "@/hooks";

type KeyEffectFileSelectorProps = {
  profileIndex: number;
  keybindIndex: number;
};

type PlaySoundEffect = Extract<
  SoundEffect,
  { type: RustSoundEffect.PlaySound }
>;

/**
 * Sub-component to uses for each sound effect config that needs a file associated
 * @param {KeybindProps} props component prop
 */
function KeyEffectFileSelector({
  profileIndex,
  keybindIndex,
}: KeyEffectFileSelectorProps) {
  const [config, setConfig] = useConfig();

  // Store selected file (absolute path) into config, null clears it
  const updateFile = useCallback(
    (path: string | null) => {
      if (!config) return;

      // Deep copy: the cached config must not be mutated
      const new_config = structuredClone(config);
      const soundEffect = new_config.profiles[profileIndex].keybind_config[
        keybindIndex
      ].sound_effect as PlaySoundEffect;

      soundEffect.path = path;
      // Basename, whatever the OS path separator
      soundEffect.name = path?.split(/[\\/]/).pop() ?? "";
      // Not provided by the native dialog
      soundEffect.lastModified = null;
      setConfig(new_config);
    },
    [config, profileIndex, keybindIndex],
  );

  // Native dialog: unlike a webview file input, it returns the absolute path
  const handleSelectFile = useCallback(async () => {
    const path = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Audio", extensions: ["wav"] }],
    });

    // null: dialog cancelled, keep current file
    if (path) updateFile(path);
  }, [updateFile]);

  if (!config) return null;

  const soundEffect = config.profiles[profileIndex].keybind_config[keybindIndex]
    .sound_effect as PlaySoundEffect;

  return (
    <TextInput
      readOnly
      placeholder="Select a sound file"
      value={soundEffect.name}
      title={soundEffect.path ?? undefined}
      onClick={handleSelectFile}
      styles={{ input: { cursor: "pointer" } }}
      rightSection={
        soundEffect.path ? (
          <CloseButton
            size="sm"
            aria-label="Clear sound file"
            onClick={(event) => {
              // Don't reopen the dialog
              event.stopPropagation();
              updateFile(null);
            }}
          />
        ) : null
      }
    />
  );
}

export default KeyEffectFileSelector;
