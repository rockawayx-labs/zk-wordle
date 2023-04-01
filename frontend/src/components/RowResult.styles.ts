import { createStyles } from "@mantine/core";

export default createStyles((theme) => ({
  cell: {
    height: "4rem",
    textAlign: "center",
    border: "3px solid transparent",
    fontSize: theme.fontSizes.xl,
    borderRadius: theme.radius.xs,
    textTransform: "uppercase",
  },
  present: {
    borderColor: theme.colors.orange[5],
  },
  miss: {
    borderColor: theme.colors.gray[4],
  },
  correct: {
    borderColor: theme.colors.teal[5],
  },
}));
