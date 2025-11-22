import { useConfig, useKeyboard } from "@/hooks";
import { KeyboardStateType } from "@/types";
import { Group, Kbd, Loader, Paper, Popover, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";
import { useCallback, useRef, useState } from "react";

type KeybindProps = {
  profileIndex: number;
  keybindIndex: number;
};

function Keybind({ profileIndex, keybindIndex }: KeybindProps) {
  const [popOverOpened, setPopOverOpened] = useState(false);
  const [config, setConfig] = useConfig();
  const isKeybindListening = useRef<boolean>(false);
  const lastKeyboardState = useRef<KeyboardStateType>();

  if (!config) return null;

  const isKeybindAssigned: boolean =
    config.profiles[profileIndex].keybind_config[keybindIndex].keycode_list
      .length > 0;

  const onKeyboardStateChanged = useCallback(
    (keyboardState: KeyboardStateType) => {
      // Do nothing if we're not waiting for input key
      if (!isKeybindListening.current) return;

      // If at least one key was pressed and now there is less key pressed OR there is 3 key pressed
      if (
        lastKeyboardState.current &&
        (lastKeyboardState.current.keyPressedList.length >
          keyboardState.keyPressedList.length ||
          keyboardState.keyPressedList.length == 3)
      ) {
        const newKeybind =
          lastKeyboardState.current.keyPressedList.length >
          keyboardState.keyPressedList.length
            ? lastKeyboardState.current.keyPressedList
            : keyboardState.keyPressedList;

        // Assign keybind to proper profile / effect
        const new_config = { ...config };
        new_config.profiles[profileIndex].keybind_config[
          keybindIndex
        ].keycode_list = newKeybind;
        setConfig(new_config);

        // Close popover
        handleOnChange(false);
      }
      lastKeyboardState.current = keyboardState;
    },
    [config],
  );

  useKeyboard(onKeyboardStateChanged);

  const handleOnChange = useCallback(
    (opened: boolean) => {
      isKeybindListening.current = opened;
      lastKeyboardState.current = undefined;
      setPopOverOpened(opened);
      invoke("set_keybind_listening_state", { isListening: opened });
    },
    [setPopOverOpened],
  );

  return (
    <Popover
      width="200px"
      offset={5}
      position="top"
      withArrow
      shadow="xl"
      withOverlay
      opened={popOverOpened}
      onChange={handleOnChange}
    >
      <Popover.Target>
        <Paper
          withBorder={true}
          p="xs"
          style={{ cursor: "pointer" }}
          onClick={() => handleOnChange(!popOverOpened)}
        >
          <Group justify="center">
            <p>Key(s) associated</p>
            <Kbd
              style={{ color: isKeybindAssigned ? "white" : "red" }}
              size="lg"
            >
              {config.profiles[profileIndex].keybind_config[keybindIndex]
                .keycode_list[0] ?? "???"}
            </Kbd>
            <Kbd
              style={{ color: isKeybindAssigned ? "white" : "red" }}
              size="lg"
              hidden={
                config.profiles[profileIndex].keybind_config[keybindIndex]
                  .keycode_list.length < 2
              }
            >
              {config.profiles[profileIndex].keybind_config[keybindIndex]
                .keycode_list[1] ?? "???"}
            </Kbd>
            <Kbd
              style={{ color: isKeybindAssigned ? "white" : "red" }}
              size="lg"
              hidden={
                config.profiles[profileIndex].keybind_config[keybindIndex]
                  .keycode_list.length < 3
              }
            >
              {config.profiles[profileIndex].keybind_config[keybindIndex]
                .keycode_list[2] ?? "???"}
            </Kbd>
            {/* <ThemeIcon color="red" size="sm" radius="lg">
                {isKeybindAssigned ? <IconCheck/> : <IconX/>}
            </ThemeIcon> */}
          </Group>
        </Paper>
      </Popover.Target>
      <Popover.Dropdown>
        <Group>
          <Text size="sm">Press key(s)</Text>
          <Loader type="dots" />
        </Group>
      </Popover.Dropdown>
    </Popover>
  );
}

export default Keybind;
