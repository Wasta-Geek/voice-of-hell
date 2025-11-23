import { Stack } from "@mantine/core";

import { ProfileKeyEffectList, ProfileManagement } from "@/components/profile";

/**
 * Profile tab
 */
function Profiles() {
  return (
    <Stack align="center">
      <ProfileManagement />
      <ProfileKeyEffectList />
    </Stack>
  );
}

export default Profiles;
