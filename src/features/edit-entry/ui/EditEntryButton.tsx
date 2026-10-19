import { useState, type FormEvent, type ReactNode } from "react";

import { EntryFields, formFromEntry, toInput, type EntryFormValues } from "@/entities/entry";
import { errorMessage, isApiError, saveEntry, type EntryDetail } from "@/shared/api";
import type { ConflictHandler } from "@/shared/lib";
import { Button, ErrorText, Modal } from "@/shared/ui";

interface Props {
  entry: EntryDetail;
  onSaved: () => void;
  onConflict: ConflictHandler;
  renderPasswordTools?: (setPassword: (value: string) => void) => ReactNode;
}

export function EditEntryButton({ entry, onSaved, onConflict, renderPasswordTools }: Props) {
  const [open, setOpen] = useState(false);
  const [values, setValues] = useState<EntryFormValues>(() => formFromEntry(entry));
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const openEditor = () => {
    setValues(formFromEntry(entry));
    setError(null);
    setOpen(true);
  };
  const close = () => setOpen(false);

  async function submit(force: boolean) {
    setBusy(true);
    try {
      await saveEntry(toInput(values, entry.id), force);
      close();
      onSaved();
    } catch (e) {
      if (isApiError(e, "external_modification")) onConflict(() => submit(true));
      else setError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <Button onClick={openEditor}>Edit</Button>
      {open && (
        <Modal title="Edit entry" onClose={close}>
          <form
            onSubmit={(e: FormEvent) => {
              e.preventDefault();
              void submit(false);
            }}
          >
            <EntryFields
              values={values}
              onChange={(patch) => setValues((v) => ({ ...v, ...patch }))}
              passwordHint="Leave empty to keep the current password."
              passwordTools={renderPasswordTools?.((password) => setValues((v) => ({ ...v, password })))}
            />
            <ErrorText message={error} />
            <div className="ui-modal__actions">
              <Button onClick={close}>Cancel</Button>
              <Button variant="primary" type="submit" disabled={busy || !values.title.trim()}>
                Save
              </Button>
            </div>
          </form>
        </Modal>
      )}
    </>
  );
}
