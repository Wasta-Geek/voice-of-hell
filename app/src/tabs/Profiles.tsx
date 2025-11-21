import { Stack } from '@mantine/core';

import { ProfileKeyEffectList, ProfileManagement } from '@/components/profile';


// Tab: Profile
function Profiles() {

  return (
    <Stack align='center'>
      <ProfileManagement/>
      <ProfileKeyEffectList/>
    </Stack>
  );
}

export default Profiles;