import { PinInput, Stack } from "@mantine/core";
import { useFormContext } from "../hooks/useWordleForm";
import useStyles from "./Row.styles";
export interface RowProps {
  disabled: boolean;
}

function getCharAtIndex(str: string, index: number) {
  try {
    return str.charAt(index);
  } catch (e) {
    return "";
  }
}

export function Row({ disabled }: RowProps) {
  const { classes } = useStyles();
  const form = useFormContext();

  return (
    <Stack>
      <PinInput
        length={5}
        spacing="md"
        autoFocus
        required
        disabled={disabled}
        classNames={{ input: classes.input }}
        placeholder=""
        inputMode="text"
        type={RegExp("[a-zA-Z]")}
        value={
          form.values.first +
          form.values.second +
          form.values.third +
          form.values.fourth +
          form.values.fifth
        }
        onChange={(input) =>
          form.setValues({
            first: getCharAtIndex(input, 0),
            second: getCharAtIndex(input, 1),
            third: getCharAtIndex(input, 2),
            fourth: getCharAtIndex(input, 3),
            fifth: getCharAtIndex(input, 4),
          })
        }
      />
    </Stack>
  );
}
