import { useEffect, useState } from "react";

import { errorMessage, revealPassword } from "@/shared/api";
import { Button } from "@/shared/ui";

import "./secret.css";

const HIDE_AFTER_MS = 15_000;

/** Shows the password only on demand, and hides (and forgets) it again after 15 s. */
export function RevealPassword({ entryId }: { entryId: string }) {
  const [value, setValue] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setValue(null);
    setError(null);
  }, [entryId]);

  useEffect(() => {
    if (value === null) return;
    const t = setTimeout(() => setValue(null), HIDE_AFTER_MS);
    return () => clearTimeout(t);
  }, [value]);

  async function show() {
    try {
      setValue(await revealPassword(entryId));
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  const shown = value !== null;
  return (
    <>
      <code className="secret">{shown ? value : (error ?? "••••••••••••")}</code>
      <Button
        size="sm"
        variant="ghost"
        iconOnly
        icon={shown ? "eye-off" : "eye"}
        onClick={shown ? () => setValue(null) : show}
        title={shown ? "Hide password" : "Show password (hides after 15 s)"}
        aria-label={shown ? "Hide password" : "Show password"}
      />
    </>
  );
}
