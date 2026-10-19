import { useEffect, useState } from "react";

import { errorMessage, revealPassword } from "@/shared/api";
import { Button } from "@/shared/ui";

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

  return (
    <>
      <code className="secret">{value ?? (error ?? "••••••••••")}</code>
      {value === null ? <Button onClick={show}>Show</Button> : <Button onClick={() => setValue(null)}>Hide</Button>}
    </>
  );
}
