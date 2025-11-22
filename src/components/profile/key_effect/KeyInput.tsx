import { Modal } from "@mantine/core"
import { useDisclosure } from "@mantine/hooks";

type KeyEffectKeyInputProps = {
    profileIndex: number,
    keybindIndex: number
}

function KeyEffectKeyInput({ profileIndex, keybindIndex }: KeyEffectKeyInputProps) {
    const [opened, { open, close }] = useDisclosure(false);
    const { data: keyPressedList } = useGetKeyPressedList();


    return (
        <Modal withinPortal={false} opened={opened} onClose={close} withCloseButton={true} size="auto" centered>
            <NewProfile closeCallback={close} />
        </Modal>
    );
}