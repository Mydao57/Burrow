import { useState, type FormEvent, type ReactNode } from "react";

import { emptyEntryForm, EntryFields, toInput, type EntryFormValues } from "@/entities/entry";
import { errorMessage, isApiError, saveEntry } from "@/shared/api";
import type { ConflictHandler } from "@/shared/lib";
import { Button, ErrorText, Modal } from "@/shared/ui";

interface Props {
  onSaved: (id: string) => void;
  onConflict: ConflictHandler;
  renderPasswordTools?: (setPassword: (value: string) => void) => ReactNode;
}

export function AddEntryButton({ onSaved, onConflict, renderPasswordTools }: Props) {
  const [open, setOpen] = useState(false);
  const [values, setValues] = useState<EntryFormValues>(emptyEntryForm);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const close = () => {
    setOpen(false);
    setValues(emptyEntryForm);
    setError(null);
  };

  async function submit(force: boolean) {
    setBusy(true);
    try {
      const saved = await saveEntry(toInput(values), force);
      close();
      onSaved(saved.id);
    } catch (e) {
      if (isApiError(e, "external_modification")) onConflict(() => submit(true));
      else setError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <Button variant="primary" icon="plus" onClick={() => setOpen(true)}>
        New
      </Button>
      {open && (
        <Modal title="New entry" onClose={close}>
          <form
            onSubmit={(e: FormEvent) => {
              e.preventDefault();
              void submit(false);
            }}
          >
            <EntryFields
              values={values}
              onChange={(patch) => setValues((v) => ({ ...v, ...patch }))}
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
