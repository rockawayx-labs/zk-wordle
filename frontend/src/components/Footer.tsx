import { Box, Divider, Group } from "@mantine/core";
import { RockawayX } from "./RockawayX";

export function Footer() {
  return (
    <>
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
