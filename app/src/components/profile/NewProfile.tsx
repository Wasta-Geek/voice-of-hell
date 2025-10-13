import { ActionIcon, Stack, TextInput } from '@mantine/core';
import { IconDeviceFloppy } from '@tabler/icons-react';
import { ChangeEvent, useCallback, useState } from 'react';
import { useGetConfig, useUpdateConfig } from '../../hooks/useConfig';

function NewProfile({closeCallback}: {closeCallback: () => void}) {
  const [newProfileName, setNewProfileName] = useState<string>('');
  const [errorString, setErrorString] = useState<string>('');
  const { data: config } = useGetConfig();
  const updateConfig = useUpdateConfig();
  const isSaveButtonDisabled = newProfileName.length == 0 || errorString.length > 0;

  // Handler that manages when a profile name is given in the text input
  const handleProfileName = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    const profileName = event.currentTarget.value;
    const profileAlreadyExists = config.profiles.find((profile) => profile.name == profileName);
  
    if (profileAlreadyExists) {
      setErrorString("Profile already exists");
    }
    else {
      setErrorString("");
    }
    setNewProfileName(profileName);
  }, [config]);

  // Handler that manages saving profile
  const handleSaveProfile = useCallback(() => {
    // Create new profile
    const newProfile = { name: newProfileName, keymap_config: { keymap_effect_config: {} } }
    // Add new profile to existing config
    config.profiles.push(newProfile);
    // Save config with added profile
    updateConfig.mutateAsync(config).then(() => closeCallback());
  }, [newProfileName, closeCallback]);


  return (
      <Stack align="center" justify="center" gap="lg">
        <TextInput label="Profile name" placeholder="My Profile" value={newProfileName} onChange={handleProfileName} error={errorString}></TextInput>
        <ActionIcon color="gray" radius="md" size="md" disabled={isSaveButtonDisabled} onClick={handleSaveProfile}><IconDeviceFloppy/></ActionIcon>
      </Stack>
  );
}

export default NewProfile;