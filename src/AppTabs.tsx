import { Tabs } from "@mantine/core";

import Devices from "./tabs/Devices";
import Profiles from "./tabs/Profiles";

function AppTabs() {
  return (
    <Tabs>
      <Tabs.List>
        <Tabs.Tab value="devices">Devices</Tabs.Tab>
        <Tabs.Tab value="profiles">Profiles</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="devices">
        <Devices />
      </Tabs.Panel>
      <Tabs.Panel value="profiles">
        <Profiles />
      </Tabs.Panel>
    </Tabs>
  );
}

export default AppTabs;
