import { createStyles, rem } from "@mantine/core";

export const ICON_SIZE = rem(40);

export default createStyles((theme) => ({
  card: {
    position: "relative",
    overflow: "visible",
    padding: theme.spacing.xl,
    paddingTop: `calc(${theme.spacing.md} * 1.5 + ${ICON_SIZE} / 3)`,
  },

  icon: {
    position: "absolute",
    top: `calc(-${ICON_SIZE} / 3)`,
    left: `calc(50% - ${ICON_SIZE} / 2)`,
  },

  title: {
    fontFamily: `Greycliff CF, ${theme.fontFamily}`,
    lineHeight: 1.5,
    fontSize: '1.8rem'
  },
  subTitle: {
    fontSize: '1.2rem'
  },
  breakAll: {
    wordBreak: "break-all",
  },
}));
