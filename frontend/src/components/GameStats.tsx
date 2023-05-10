import {
  Progress,
  Text,
  Group,
  Paper,
  ThemeIcon,
  PaperProps,
  Title,
  Stack,
  Card,
  Anchor,
} from "@mantine/core";
import { IconBrackets } from "@tabler/icons-react";
import useStyles, { ICON_SIZE } from "./GameStats.styles";
import { CONTRACT_ADDRESS } from "../constants";

export interface GameStatsProps extends PaperProps {
  turn: number;
  commitment?: string;
  imageId?: string;
}

export function GameStats({
  turn,
  commitment,
  imageId,
  ...others
}: GameStatsProps) {
  const { classes } = useStyles();

  return (
    <Paper
      radius="md"
      withBorder
      className={classes.card}
      mt={`calc(${ICON_SIZE} / 3)`}
      {...others}
    >
      <ThemeIcon className={classes.icon} size={ICON_SIZE} radius={ICON_SIZE}>
        <IconBrackets size="1.5rem" stroke={1.5} />
      </ThemeIcon>

      <Title ta="center" fw={700} className={classes.title}>
        ZK Wordle Challenge
      </Title>
      <Title order={2} c="dimmed" ta="center" className={classes.subTitle}>
        That is verifiably fair
      </Title>

      <Card mt="xl" withBorder>
        <Stack spacing={0}>
          <Text fw={700}>Contract address</Text>
          <Anchor
            size="sm"
            fw={700}
            href={`https://mumbai.polygonscan.com/address/${CONTRACT_ADDRESS}`}
            rel="noopener noreferrer"
            target="_blank"
          >
            {CONTRACT_ADDRESS}
          </Anchor>
        </Stack>

        <Stack mt="md" spacing={0}>
          <Text fw={700}>Commitment to the guessed word</Text>
          <Text fz="sm" color="dimmed" className={classes.breakAll}>
            {commitment ?? "N/A"}
          </Text>
        </Stack>

        <Stack mt="md" spacing={0}>
          <Text fw={700}>Prover image ID</Text>
          <Text fz="sm" color="dimmed" className={classes.breakAll}>
            {imageId ?? "N/A"}
          </Text>
        </Stack>
      </Card>

      <Group position="apart" mt="xs">
        <Text fz="sm" color="dimmed">
          Turn
        </Text>
        <Text fz="sm" color="dimmed">
          {turn}/6
        </Text>
      </Group>

      <Progress value={(turn / 6) * 100} mt={5} />
    </Paper>
  );
}
