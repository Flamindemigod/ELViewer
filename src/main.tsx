import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
// import "@radix-ui/themes/styles.css";
import { Theme, ThemePanel } from "@radix-ui/themes";
import { ParserProvider } from "./contexts/Parser";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ParserProvider>
      <Theme
        appearance="dark"
        accentColor="ruby"
        grayColor="sage"
        scaling="95%"
      >
        <App />
        <ThemePanel />
      </Theme>
    </ParserProvider>
  </React.StrictMode>,
);
