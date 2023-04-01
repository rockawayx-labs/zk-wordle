import { Divider, Group } from "@mantine/core";
import { RockawayX } from "./RockawayX";

export function Footer() {
  return (
    <>
      <Divider my="xl" />
      <Group position="center">
        <RockawayX />
      </Group>
    </>
  );
}
