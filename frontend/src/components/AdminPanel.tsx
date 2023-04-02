import { Box, Button, Group, LoadingOverlay, Tooltip } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { showNotification } from "@mantine/notifications";
import { Icon24Hours } from "@tabler/icons-react";
import { useState } from "react";
import { extractErrorMessage } from "../utils";
import useStyles from "./AdminPanel.styles";

export function AdminPanel() {
  const { classes } = useStyles();
  const [loading, setLoading] = useState(false);
  const [admin] = useLocalStorage({
    key: "admin",
    defaultValue: false,
  });

  const handleInit = async () => {
    setLoading(true);
    try {
      const init = await fetch("/api/init", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
      });

      const data = await init.json();
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

  const handleSetImage = async () => {
    setLoading(true);
    try {
      await fetch("/api/image", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
      });

      showNotification({
        title: "Success",
        message: `Image ID was set in the contract.`,
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

  if (!admin) {
    return null;
  }

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
              onClick={handleInit}
            >
              Init Game
            </Button>
          </Tooltip>

          <Tooltip
            label="ADMIN ONLY: This sets the current builds image id into the contract, this will be used to verify the receipt."
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
              onClick={handleSetImage}
            >
              Commit Image ID
            </Button>
          </Tooltip>
        </Group>
      </Box>
    </>
  );
}
