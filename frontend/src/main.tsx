import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider
      withNormalizeCSS
      withGlobalStyles
      theme={{
        colorScheme: "light",
        fontFamily: "'Space Grotesk', sans-serif",
        defaultRadius: "xs",
      }}
    >
      <Notifications />
      <App />
    </MantineProvider>
  </React.StrictMode>
);
