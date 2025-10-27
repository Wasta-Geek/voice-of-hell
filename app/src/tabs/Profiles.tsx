import { useState } from 'react';
import { Stack } from '@mantine/core';
import { useGetConfig } from '../hooks';
import { ProfileKeyEffectList, ProfileManagement } from '../components/profile';


// Tab: Profile
function Profiles() {
  const [profileIndex, setProfileIndex] = useState<string | null>();
  const { data: config } = useGetConfig();

  return (
    <Stack align='center'>
      <ProfileManagement profileIndex={profileIndex} setProfileIndex={setProfileIndex} />
      <ProfileKeyEffectList config={config} />
    </Stack>
  );
}

export default Profiles;