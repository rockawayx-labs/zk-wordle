import { Center, SimpleGrid } from "@mantine/core";
import type { LetterFeedbackType } from "wasm-verifier";
import useStyles from "./RowResult.styles";

interface RowResultProps {
  feedback: [
    [string, LetterFeedbackType],
    [string, LetterFeedbackType],
    [string, LetterFeedbackType],
    [string, LetterFeedbackType],
    [string, LetterFeedbackType]
  ];
}

export function RowResult({ feedback }: RowResultProps) {
  const { classes, cx } = useStyles();

  return (
    <SimpleGrid cols={5} spacing="md">
      {feedback.map(([char, type], i) => (
        <Center
          key={i}
          className={cx(classes.cell, {
            [classes.correct]: type === "Correct",
            [classes.present]: type === "Present",
            [classes.miss]: type === "Miss",
          })}
        >
          {char}
        </Center>
      ))}
    </SimpleGrid>
  );
}
