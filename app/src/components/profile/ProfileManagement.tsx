import { ActionIcon, Group, Modal, Select, Stack } from "@mantine/core";
import { IconSquarePlus, IconTrash } from "@tabler/icons-react";
import { useCallback, useEffect } from "react";
import { useGetConfig, useUpdateConfig } from "../../hooks";
import { useDisclosure } from "@mantine/hooks";
import NewProfile from "./NewProfile";


type ProfileManagementProps = {
    profileIndex: string | null | undefined;
    setProfileIndex: React.Dispatch<React.SetStateAction<string | null | undefined>>;
};

function ProfileManagement({ profileIndex, setProfileIndex }: ProfileManagementProps) {
    const updateConfig = useUpdateConfig();
    const [opened, { open, close }] = useDisclosure(false);
    const { data: config } = useGetConfig();

    useEffect(() => {
        if (config?.last_profile_index_used) {
            setProfileIndex(config.last_profile_index_used);
        }
    }, [config]);

    const handleSelectProfile = useCallback((profileIndex: string | null) => {
        // Reset current profile selected
        setProfileIndex(profileIndex);
        if (config) {
            config.last_profile_index_used = profileIndex;
            // Save config with last profile used
            updateConfig.mutateAsync(config);
        }
    }, [config])

    const handleDeleteProfile = useCallback(() => {
        if (config) {
            // Remove profile from config
            config.profiles.splice(profileIndex, 1);
            // Reset current profile selected
            config.last_profile_index_used = null;
            // Save config with deleted profile
            updateConfig.mutateAsync(config);
        }
    }, [profileIndex]);

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
                    value={profileIndex}
                    onChange={handleSelectProfile}
                    size="md"
                    radius="md"
                />
                <Stack gap="xs" mt="lg">
                    <ActionIcon color="gray" radius="md" size="xs" onClick={open}><IconSquarePlus /></ActionIcon>
                    <ActionIcon color="gray" radius="md" size="xs" disabled={profileIndex == null} onClick={handleDeleteProfile}><IconTrash /></ActionIcon>
                </Stack>
            </Group>
        </div>
    );
}

export default ProfileManagement;