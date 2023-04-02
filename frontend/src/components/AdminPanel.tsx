import { Box, Button, Group, LoadingOverlay, Tooltip } from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { Icon24Hours } from "@tabler/icons-react";
import { useState } from "react";
import { extractErrorMessage } from "../utils";
import useStyles from "./AdminPanel.styles";

export function AdminPanel() {
  const { classes } = useStyles();
  const [loading, setLoading] = useState(false);

  const handleReset = async () => {
    setLoading(true);
    try {
      const response = await fetch("/api/init", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
      });
      const data = await response.json();
      showNotification({
        title: "Game reset",
        message: `New word "${data?.word}" was picked and commitment was passed to chain.`,
        color: "teal",
      });
      await new Promise((resolve) => setTimeout(resolve, 1500));
      location.reload();
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
    <>
      <LoadingOverlay visible={loading} />
      <Box className={classes.root}>
        <Group position="center" h="100%">
          <Tooltip
            label="ADMIN ONLY: This resets the current state of the game for everyone, server picks a new word and passes the commitment to chain."
            withArrow
            multiline
            width={150}
          >
            <Button
              size="xs"
              compact
              variant="outline"
              color="red"
              leftIcon={<Icon24Hours size={16} />}
              loading={loading}
              onClick={handleReset}
            >
              Reset game
            </Button>
          </Tooltip>
        </Group>
      </Box>
    </>
  );
}
