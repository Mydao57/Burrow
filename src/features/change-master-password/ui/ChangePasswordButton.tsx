import { useState, type FormEvent } from "react";

import { changeMasterPassword, errorMessage, isApiError } from "@/shared/api";
import { Button, ErrorText, Modal, TextField } from "@/shared/ui";

const MIN_LENGTH = 12;

export function ChangePasswordButton() {
  const [open, setOpen] = useState(false);
  const [current, setCurrent] = useState("");
  const [next, setNext] = useState("");
  const [confirm, setConfirm] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [done, setDone] = useState(false);

  const close = () => {
    setOpen(false);
    setCurrent("");
    setNext("");
    setConfirm("");
    setError(null);
  };

  async function submit(e: FormEvent) {
    e.preventDefault();
    if (next.length < MIN_LENGTH) return setError(`Use at least ${MIN_LENGTH} characters.`);
    if (next !== confirm) return setError("The two new passwords do not match.");
    setBusy(true);
    setError(null);
    try {
      await changeMasterPassword(current, next);
      close();
      setDone(true);
      setTimeout(() => setDone(false), 4000);
    } catch (err) {
      setError(
        isApiError(err, "wrong_password")
          ? "The current master password is incorrect."
          : isApiError(err, "external_modification")
            ? "The vault file changed on disk. Lock and unlock to reload it, then try again."
            : errorMessage(err),
      );
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <Button
        variant="ghost"
        size="sm"
        icon={done ? "check" : "key"}
        onClick={() => setOpen(true)}
        title="Change the master password"
      >
        {done ? "Password changed" : "Master password"}
      </Button>
      {open && (
        <Modal title="Change master password" onClose={close}>
          <form onSubmit={submit}>
            <TextField
              label="Current master password"
              type="password"
              value={current}
              onChange={(e) => setCurrent(e.target.value)}
              autoComplete="current-password"
              autoFocus
            />
            <TextField
              label="New master password"
              type="password"
              value={next}
              onChange={(e) => setNext(e.target.value)}
              autoComplete="new-password"
              hint="There is no recovery. Other copies of the vault file (backups, synced devices) keep the old password."
            />
            <TextField
              label="Confirm new master password"
              type="password"
              value={confirm}
              onChange={(e) => setConfirm(e.target.value)}
              autoComplete="new-password"
            />
            <ErrorText message={error} />
            <div className="ui-modal__actions">
              <Button onClick={close} disabled={busy}>Cancel</Button>
              <Button variant="primary" type="submit" disabled={busy || !current || !next || !confirm}>
                {busy ? "Re-encrypting…" : "Change password"}
              </Button>
            </div>
          </form>
        </Modal>
      )}
    </>
  );
}
