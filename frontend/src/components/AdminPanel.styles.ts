import { createStyles } from "@mantine/core";

export default createStyles((theme) => ({
  root: {
    position: "absolute",
    top: 0,
    right: 0,
    width: "auto",
    height: 40,
    maxWidth: "100%",
    paddingLeft: theme.spacing.xl,
    paddingRight: theme.spacing.md,
    backgroundColor: theme.black,
    borderBottomLeftRadius: theme.radius.lg,
  },
}));
