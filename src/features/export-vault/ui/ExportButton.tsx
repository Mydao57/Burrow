import { useState } from "react";

import { errorMessage, exportVault } from "@/shared/api";
import { Button } from "@/shared/ui";

export function ExportButton() {
  const [message, setMessage] = useState<string | null>(null);

  async function run() {
    try {
      const dest = await exportVault();
      setMessage(dest ? "Backup saved" : null);
    } catch (e) {
      setMessage(errorMessage(e));
    }
  }

  return (
    <Button onClick={run} title={message ?? "Save a copy of the encrypted vault file"}>
      {message ?? "Export backup…"}
    </Button>
  );
}
