import { ActionIcon, Group, Modal, Select, Stack } from "@mantine/core";
import { IconSquarePlus, IconTrash } from "@tabler/icons-react";
import { useCallback } from "react";
import { useDisclosure } from "@mantine/hooks";

import { useGetConfig, useUpdateConfig } from "@/hooks";
import NewProfile from "@/components/profile/NewProfile";


type ProfileManagementProps = {};

function ProfileManagement({}: ProfileManagementProps) {
    const { data: config } = useGetConfig();
    const updateConfig = useUpdateConfig();
    const [opened, { open, close }] = useDisclosure(false);

    const handleSelectProfile = useCallback((value: string | null) => {
        if (config) {
            let new_config = {...config};
            new_config.last_profile_index_used = value;
            // Save config with last profile used
            updateConfig.mutateAsync(new_config);
        }
    }, [config])

    const handleDeleteProfile = useCallback(() => {
        if (config?.last_profile_index_used) {
            const profileIndex = parseInt(config.last_profile_index_used);
            let new_config = {...config};

            // Remove profile from config
            new_config.profiles.splice(profileIndex, 1);
            // Reset current profile selected
            new_config.last_profile_index_used = null;
            // Save config with deleted profile
            updateConfig.mutateAsync(new_config);
        }
    }, [config]);

    return (
        <div>
            <Modal withinPortal={false} opened={opened} onClose={close} withCloseButton={true} size="auto" centered>
                <NewProfile closeCallback={close} />
            </Modal>
            <Group justify="center" align="center">
                <Select
                    label="Current profile"
                    placeholder="Select your profile"
                    data={config?.profiles.map((profile, index) => ({ value: String(index), label: profile.name }))}
                    value={config?.last_profile_index_used ?? null}
                    onChange={handleSelectProfile}
                    size="md"
                    radius="md"
                />
                <Stack gap="xs" mt="lg">
                    <ActionIcon color="gray" radius="md" size="xs" onClick={open}><IconSquarePlus /></ActionIcon>
                    <ActionIcon color="gray" radius="md" size="xs" disabled={config?.last_profile_index_used == null} onClick={handleDeleteProfile}><IconTrash /></ActionIcon>
                </Stack>
            </Group>
        </div>
    );
}

export default ProfileManagement;