import {
  Progress,
  Text,
  Group,
  Paper,
  ThemeIcon,
  PaperProps,
  Title,
} from "@mantine/core";
import { IconBrackets } from "@tabler/icons-react";
import useStyles, { ICON_SIZE } from "./GameStats.styles";

export interface GameStatsProps extends PaperProps {
  turn: number;
}

export function GameStats({ turn, ...others }: GameStatsProps) {
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
        <IconBrackets size="2rem" stroke={1.5} />
      </ThemeIcon>

      <Title ta="center" fw={700} className={classes.title}>
        ZK Wordle Challenge
      </Title>
      <Title order={2} c="dimmed" ta="center">
        That is verifiably fair
      </Title>

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
