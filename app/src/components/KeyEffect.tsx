import { Group, Select, SelectProps, Stack } from "@mantine/core";
import { SoundEffect, SoundEffectType } from "../types/SoundEffect";
import { useState } from "react";
import { IconFileMusic } from "@tabler/icons-react";

function KeyEffectComponent({ soundEffect }: { soundEffect: SoundEffect }) {
    const [currentSoundEffectType, setSoundEffectType] = useState<string | null>();

    function handleSelectSoundEffectType() {

    }

    const iconProps = {
        stroke: 1.5,
        color: 'currentColor',
        opacity: 0.6,
        size: 18,
    };

    const icons: Record<string, React.ReactNode> = {
        left: <IconFileMusic {...iconProps} />,
        center: <IconFileMusic {...iconProps} />,
        right: <IconFileMusic {...iconProps} />,
        justify: <IconFileMusic {...iconProps} />,
    };

    const renderSelectOption: SelectProps['renderOption'] = ({ option, checked }) => (
        <Group flex="1" gap="xs">
            {icons[option.value]}
            {option.label}
        </Group>
    );

    return (
        <Stack>
            <Select
                label="Select an effect"
                data={Object.entries(SoundEffectType).map(([value, label]) => ({ label, value }))}
                value={currentSoundEffectType}
                onChange={handleSelectSoundEffectType}
                size="md"
            />
        </Stack>
    );
}

export default KeyEffectComponent;