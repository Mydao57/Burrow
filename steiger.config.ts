import fsd from "@feature-sliced/steiger-plugin";
import { defineConfig } from "steiger";

export default defineConfig([
  ...fsd.configs.recommended,
  {
    // The slice layout is a deliberate design choice (one slice per user
    // capability), even when a slice currently has a single consumer.
    rules: { "fsd/insignificant-slice": "off" },
  },
]);
