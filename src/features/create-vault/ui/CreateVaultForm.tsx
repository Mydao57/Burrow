import { useState, type FormEvent } from "react";

import { chooseVaultFile, createVault, errorMessage, type VaultInfo } from "@/shared/api";
import { Button, ErrorText, TextField } from "@/shared/ui";

const MIN_LENGTH = 12;

interface Props {
  onCreated: (info: VaultInfo) => void;
}

export function CreateVaultForm({ onCreated }: Props) {
  const [path, setPath] = useState<string | null>(null);
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function choose() {
    try {
      const picked = await chooseVaultFile("create");
      if (picked) setPath(picked);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function submit(e: FormEvent) {
    e.preventDefault();
    if (password.length < MIN_LENGTH) return setError(`Use at least ${MIN_LENGTH} characters.`);
    if (password !== confirm) return setError("The two passwords do not match.");
    setBusy(true);
    setError(null);
    try {
      const info = await createVault(password);
      setPassword("");
      setConfirm("");
      onCreated(info);
    } catch (err) {
      setError(errorMessage(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit}>
      <p className="muted">{path ?? "Pick where the encrypted file will live (e.g. a synced folder)."}</p>
      <Button onClick={choose}>{path ? "Choose another location…" : "Choose location…"}</Button>
      <div className="spacer" />
      <TextField
        label="Master password"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        autoComplete="new-password"
        hint="There is no recovery: if you forget it, the vault cannot be opened."
        disabled={!path}
      />
      <TextField
        label="Confirm master password"
        type="password"
        value={confirm}
        onChange={(e) => setConfirm(e.target.value)}
        autoComplete="new-password"
        disabled={!path}
      />
      <ErrorText message={error} />
      <Button variant="primary" type="submit" disabled={!path || !password || busy}>
        {busy ? "Creating…" : "Create vault"}
      </Button>
    </form>
  );
}
