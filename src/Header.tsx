import { Grid, Slider, Center } from '@mantine/core';

import "./App.css";

function Header() {
  return (
    <Grid justify="center" align="center">
      <Grid.Col span="content" offset={4}>
        <p>Voice of Hell</p>
      </Grid.Col>
      <Grid.Col span={2} offset={2}>
          <Center>Volume</Center>
          <Slider
            color="green"
            size="md"
            defaultValue={100}
            thumbSize={16}
          />
      </Grid.Col>
    </Grid>
  );
}

export default Header;