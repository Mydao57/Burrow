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

  const label =
    status === "copied" ? "Copied · clears in 20 s" : status === "error" ? (error ?? "Copy failed") : `Copy ${field}`;

  return (
    <Button
      size="sm"
      variant="ghost"
      iconOnly={status === "idle"}
      icon={status === "copied" ? "check" : "copy"}
      onClick={copy}
      title={label}
      aria-label={label}
      className={status === "copied" ? "ui-btn--ok" : undefined}
    >
      {status === "copied" ? "Copied" : status === "error" ? "Failed" : null}
    </Button>
  );
}
