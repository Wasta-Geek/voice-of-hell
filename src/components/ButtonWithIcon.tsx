import { Button } from "@mantine/core";
import { type Icon as TablerIcon } from "@tabler/icons-react";

type ButtonWithIconProps = {
  text: string;
  Icon: TablerIcon;
  onClick: React.MouseEventHandler<HTMLButtonElement>;
};

/**
 * App header component
 * @param {ButtonWithIconProps} props component prop
 */
function ButtonWithIcon({ text, Icon, onClick }: ButtonWithIconProps) {
  const icon = <Icon />;

  return (
    <Button
      color="rgba(255, 255, 255, 0)"
      radius="md"
      leftSection={icon}
      onClick={onClick}
    >
      {text}
    </Button>
  );
}

export default ButtonWithIcon;
