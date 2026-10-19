import { useEffect, useState } from "react";

import { copyPassword, copyUsername, errorMessage } from "@/shared/api";
import { Button } from "@/shared/ui";

interface Props {
  entryId: string;
  field: "password" | "username";
}

export function CopyButton({ entryId, field }: Props) {
  const [status, setStatus] = useState<"idle" | "copied" | "error">("idle");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (status === "idle") return;
    const t = setTimeout(() => setStatus("idle"), 3000);
    return () => clearTimeout(t);
  }, [status]);

  async function copy() {
    try {
      await (field === "password" ? copyPassword(entryId) : copyUsername(entryId));
      setStatus("copied");
    } catch (e) {
      setError(errorMessage(e));
      setStatus("error");
    }
  }

  return (
    <Button onClick={copy} title={status === "error" ? (error ?? undefined) : undefined}>
      {status === "copied" ? "Copied · clears in 20 s" : status === "error" ? "Copy failed" : "Copy"}
    </Button>
  );
}
