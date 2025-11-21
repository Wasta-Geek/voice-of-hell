import { ActionIcon, Stack, TextInput } from '@mantine/core';
import { IconDeviceFloppy } from '@tabler/icons-react';
import { ChangeEvent, useCallback, useState } from 'react';
import { useConfig } from '../../hooks';

function NewProfile({ closeCallback }: { closeCallback: () => void }) {
  const [newProfileName, setNewProfileName] = useState<string>('');
  const [errorString, setErrorString] = useState<string>('');
  const [config, setConfig] = useConfig();
  const isSaveButtonDisabled = newProfileName.length == 0 || errorString.length > 0;

  // Handler that manages when a profile name is given in the text input
  const handleProfileName = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    const profileName = event.currentTarget.value;
    const profileAlreadyExists = config?.profiles.find((profile) => profile.name == profileName);

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
    // Check if a profile is currently used
    if (config) {
      // Create new profile
      const newProfile = { name: newProfileName, keybind_config: [] }
      // Add new profile to existing config
      config.profiles.push(newProfile);
      // Save config with added profile
      setConfig(config).then(() => closeCallback());
    }
  }, [newProfileName, closeCallback]);


  return (
    <Stack align="center" justify="center" gap="lg">
      <TextInput label="Profile name" placeholder="My Profile" value={newProfileName} onChange={handleProfileName} error={errorString}></TextInput>
      <ActionIcon color="gray" radius="md" size="md" disabled={isSaveButtonDisabled} onClick={handleSaveProfile}><IconDeviceFloppy /></ActionIcon>
    </Stack>
  );
}

export default NewProfile;