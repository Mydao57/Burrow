import { useCallback, useState, type FormEvent, type ReactNode } from "react";

import { errorMessage, reloadVault } from "@/shared/api";
import type { ConflictHandler } from "@/shared/lib";
import { Button, ErrorText, Modal, TextField } from "@/shared/ui";

/**
 * Handles "the vault file changed on disk" (e.g. a sync tool delivered a new
 * version). The user chooses between reloading the disk version (losing the
 * pending change) and overwriting it.
 */
export function useConflictResolver(onReloaded: () => void): { onConflict: ConflictHandler; dialog: ReactNode } {
  const [pending, setPending] = useState<(() => Promise<void>) | null>(null);
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const onConflict = useCallback<ConflictHandler>((forceRetry) => setPending(() => forceRetry), []);

  const close = () => {
    setPending(null);
    setPassword("");
    setError(null);
  };

  async function overwrite() {
    if (!pending) return;
    setBusy(true);
    try {
      await pending();
      close();
    } catch (e) {
      setError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  }

  async function reload(e: FormEvent) {
    e.preventDefault();
    setBusy(true);
    try {
      await reloadVault(password);
      close();
      onReloaded();
    } catch (err) {
      setError(errorMessage(err));
    } finally {
      setBusy(false);
    }
  }

  const dialog = pending ? (
    <Modal title="Vault changed on disk" onClose={close}>
      <p>
        The vault file was modified outside Burrow since you opened it (a sync tool, another device…). Saving now would
        overwrite those changes.
      </p>
      <form onSubmit={reload}>
        <TextField
          label="Master password (to reload the disk version)"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          autoComplete="current-password"
        />
        <ErrorText message={error} />
        <div className="ui-modal__actions">
          <Button onClick={close} disabled={busy}>Cancel</Button>
          <Button variant="danger" onClick={overwrite} disabled={busy}>Overwrite disk version</Button>
          <Button variant="primary" type="submit" disabled={busy || !password}>Reload disk version</Button>
        </div>
      </form>
    </Modal>
  ) : null;

  return { onConflict, dialog };
}
