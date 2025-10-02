// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import '@mantine/core/styles.css';

import { MantineProvider, AppShell } from '@mantine/core';
import { theme } from './theme';

import Header from './Header';
import AppTabs from './AppTabs';
import { QueryClientProvider, QueryClient } from '@tanstack/react-query';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={ queryClient }>
      <MantineProvider theme={theme} defaultColorScheme="dark">
        <AppShell header={{ height: 60 }}  >
            <AppShell.Header>
                <Header/>
            </AppShell.Header>
          <AppShell.Main>
            <AppTabs/>
          </AppShell.Main>
        </AppShell>
      </MantineProvider>
  </QueryClientProvider>
  );
}

export default App;