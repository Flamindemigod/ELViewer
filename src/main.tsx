import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
// import "@radix-ui/themes/styles.css";
import { Theme, ThemePanel } from "@radix-ui/themes";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Theme appearance="dark" accentColor="ruby" grayColor="sage" scaling="95%">
      <App />
      <ThemePanel />
    </Theme>
  </React.StrictMode>,
);
