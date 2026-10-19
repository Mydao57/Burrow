import { useEffect, useState, type FormEvent } from "react";

import { chooseVaultFile, errorMessage, unlockVault, vaultStatus, type VaultInfo } from "@/shared/api";
import { Button, ErrorText, TextField } from "@/shared/ui";

interface Props {
  onUnlocked: (info: VaultInfo) => void;
}

export function UnlockForm({ onUnlocked }: Props) {
  const [path, setPath] = useState<string | null>(null);
  const [password, setPassword] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    vaultStatus().then((s) => setPath(s.selected_path)).catch(() => undefined);
  }, []);

  async function choose() {
    try {
      const picked = await chooseVaultFile("open");
      if (picked) setPath(picked);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function submit(e: FormEvent) {
    e.preventDefault();
    setBusy(true);
    setError(null);
    try {
      const info = await unlockVault(password);
      setPassword("");
      onUnlocked(info);
    } catch (err) {
      setError(errorMessage(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit}>
      <p className="muted">{path ?? "No vault file selected"}</p>
      <Button onClick={choose}>{path ? "Choose another file…" : "Open vault file…"}</Button>
      <div className="spacer" />
      <TextField
        key={path ?? "no-file"} // remount so autoFocus applies once a file is selected
        label="Master password"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        autoComplete="current-password"
        disabled={!path}
        autoFocus
      />
      <ErrorText message={error} />
      <Button variant="primary" type="submit" disabled={!path || !password || busy}>
        {busy ? "Unlocking…" : "Unlock"}
      </Button>
    </form>
  );
}
