import { Alert, Button, Stack } from "@mantine/core";
import { matches } from "@mantine/form";
import { showNotification } from "@mantine/notifications";
import { IconMoodSad, IconMoodTongueWink } from "@tabler/icons-react";
import { useState } from "react";
import { LetterFeedbackType } from "wasm-verifier";
import {
  FormProvider,
  useForm,
  WordleFormValues,
} from "../hooks/useWordleForm";
import { extractErrorMessage, objectKeys } from "../utils";
import { Verifier } from "../verifier";
import { GameStats } from "./GameStats";
import { Row } from "./Row";
import { RowResult } from "./RowResult";
import type { ContractData } from "../App";

type Feedback = [
  [string, LetterFeedbackType],
  [string, LetterFeedbackType],
  [string, LetterFeedbackType],
  [string, LetterFeedbackType],
  [string, LetterFeedbackType]
];

interface GameProps {
  contractData?: ContractData;
}

export function Game({ contractData }: GameProps) {
  const form = useForm({
    initialValues: {
      first: "",
      second: "",
      third: "",
      fourth: "",
      fifth: "",
    },
    validate: {
      first: matches(/^[a-zA-Z]$/),
      second: matches(/^[a-zA-Z]$/),
      third: matches(/^[a-zA-Z]$/),
      fourth: matches(/^[a-zA-Z]$/),
      fifth: matches(/^[a-zA-Z]$/),
    },
  });
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<"playing" | "lost" | "won">("playing");
  const [turns, setTurns] = useState<Feedback[]>([]);

  const handleSubmit = async (formValues: WordleFormValues) => {
    if (!contractData) {
      showNotification({
        title: "Not ready",
        message: "Contract data is not loaded",
        color: "orange",
      });
      return;
    }
    setLoading(true);
    try {
      const chars = objectKeys(formValues).map((key) => formValues[key]);
      const word = chars.join("").toLowerCase();

      const guessResponse = await fetch("/api/guess", {
        method: "POST",
        body: JSON.stringify({ guess: word }),
        headers: {
          "Content-Type": "application/json",
        },
      });
      const { receipt } = await guessResponse.json();

      const verifier = new Verifier();
      const data = await verifier.verify(
        receipt,
        contractData.imageId,
        contractData.commitment
      );
      if (!data.success) {
        throw new Error(data.error);
      }
      const feedback = chars
        .map((char, i) => [char, data.state.feedback[i] || "Miss"])
        .slice(0, 5) as Feedback;
      setTurns((turns) => [...turns, feedback]);

      if (feedback.every(([, feedback]) => feedback === "Correct")) {
        setStatus("won");
      } else if (turns.length >= 5) {
        setStatus("lost");
      }

      form.reset();
    } catch (e) {
      showNotification({
        title: "Error",
        message: extractErrorMessage(e),
        color: "red",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Stack>
      <GameStats
        commitment={contractData?.commitment}
        imageId={contractData?.imageId}
        turn={turns.length}
        mb="xl"
      />

      {status === "lost" && (
        <Alert
          fw={700}
          title="Game Over!"
          color="red"
          variant="filled"
          icon={<IconMoodSad />}
        >
          You lost!
        </Alert>
      )}

      {status === "won" && (
        <Alert
          fw={700}
          title="Game Over!"
          color="teal"
          variant="filled"
          icon={<IconMoodTongueWink />}
        >
          You won!
        </Alert>
      )}

      {turns.map((feedback, i) => (
        <RowResult key={i} feedback={feedback} />
      ))}

      <FormProvider form={form}>
        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack>
            <Row disabled={loading || status !== "playing"} />
            <Button
              type="submit"
              loading={loading}
              disabled={status !== "playing"}
              uppercase
            >
              Submit
            </Button>
          </Stack>
        </form>
      </FormProvider>
    </Stack>
  );
}
