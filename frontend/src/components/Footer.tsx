import { Alert, Box, Divider, Group, Text, Image } from "@mantine/core";
import { IconExclamationCircle } from "@tabler/icons-react";
import { RockawayX } from "./RockawayX";
import RiscZeroLogo from "../assets/risc-zero-logo.png";

export function Footer() {
  return (
    <>
      <Alert
        mt="sm"
        title="Disclaimer"
        icon={<IconExclamationCircle size="1rem" />}
        color="orange"
      >
        <Text color="orange">
          First guess might incorrectly re-render page, please try again right
          after, guessing should then continue as normal.
        </Text>
      </Alert>
      <Divider my="xl" />
      <Group position="apart">
        <Group spacing="xs">
          <Text size="sm" color="dimmed">
            Built by
          </Text>
          <Box
            component="a"
            href="https://rockawayx.com"
            target="_blank"
            rel="noreferrer"
            sx={{
              textDecoration: "none",
              color: "inherit",
            }}
          >
            <RockawayX size={140} mt="-1rem" />
          </Box>
        </Group>
        <Group spacing="xs">
          <Text size="sm" color="dimmed">
            Powered by
          </Text>
          <Box
            component="a"
            href="https://www.risczero.com/"
            target="_blank"
            rel="noreferrer"
            sx={{
              textDecoration: "none",
              color: "inherit",
            }}
          >
            <Image height="2.2rem" src={RiscZeroLogo} />
          </Box>
        </Group>
      </Group>
    </>
  );
}
