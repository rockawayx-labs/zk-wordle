import { SimpleGrid, TextInput } from "@mantine/core";
import { useFormContext } from "../hooks/useWordleForm";
import useStyles from "./Row.styles";

export interface RowProps {
  disabled: boolean;
}

export function Row({ disabled }: RowProps) {
  const { classes } = useStyles();
  const form = useFormContext();

  return (
    <SimpleGrid cols={5} spacing="md">
      {["first", "second", "third", "fourth", "fifth"].map((order, i) => (
        <TextInput
          key={order}
          autoFocus={i === 0}
          required
          disabled={disabled}
          minLength={1}
          maxLength={1}
          classNames={{ input: classes.input }}
          {...form.getInputProps(order)}
        />
      ))}
    </SimpleGrid>
  );
}
