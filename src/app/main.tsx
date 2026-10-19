import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { initTheme } from "@/features/theme-switcher";

import { App } from "./App";
import "./styles/global.css";

initTheme(); // before the first render, so there is no flash of the wrong theme

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
