import { ActionIcon, Grid, Modal, Select, Stack } from '@mantine/core';
import { useCallback, useState } from 'react';
import { IconSquarePlus, IconTrash } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';

import { NewProfileComponent } from '../components/index';
import { useGetConfig, useUpdateConfig } from '../hooks/Config';

function Profiles() {
  const [currentProfileIndex, setProfile] = useState<string | null>('');
  const [opened, {open, close}] = useDisclosure(false);
  const { data: config } = useGetConfig();
  const updateConfig = useUpdateConfig();

  const handleDeleteProfile = useCallback(() => {
    // Remove profile from config
    config.profiles.splice(currentProfileIndex, 1);
    // Reset current profile selected
    setProfile(null);
    // Save config with deleted profile
    updateConfig.mutateAsync(config);
  }, [currentProfileIndex]);

  return (
    <main className="container">
      <div>
        <Modal withinPortal={false} opened={opened} onClose={close} withCloseButton={true} size="auto" centered>
          <NewProfileComponent closeCallback={close}/>
        </Modal>
        <Grid justify="center" align="stretch">
          <Grid.Col span="content">
            <Select
              label="Current profile"
              placeholder="Select your profile"
              data={config?.profiles.map((profile, index) => ({value: String(index), label: profile.name}))}
              value={currentProfileIndex}
              onChange={setProfile}
              size="md"
            />
          </Grid.Col>
          <Grid.Col span="content">
            <Stack gap="xs" h="100%" justify='flex-end'>
              <ActionIcon color="gray" radius="md" size="xs" onClick={open}><IconSquarePlus/></ActionIcon>
              <ActionIcon color="gray" radius="md" size="xs" disabled={currentProfileIndex == null} onClick={handleDeleteProfile}><IconTrash/></ActionIcon>
            </Stack>
          </Grid.Col>
        </Grid>
      </div>
    </main>
  );
}

export default Profiles;