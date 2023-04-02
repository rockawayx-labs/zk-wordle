import { Alert, Box, Divider, Group } from "@mantine/core";
import { IconExclamationCircle } from "@tabler/icons-react";
import { RockawayX } from "./RockawayX";

export function Footer() {
  return (
    <>
      <Alert
        mt="xl"
        title="Disclaimer"
        icon={<IconExclamationCircle />}
        color="red"
      >
        First guess might incorrectly re-render page, please try again right
        after, guessing should then continue as normal.
      </Alert>
      <Divider my="xl" />
      <Group position="center">
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
          <RockawayX />
        </Box>
      </Group>
    </>
  );
}
