import { useState } from "react";

import { deleteEntry, errorMessage, isApiError } from "@/shared/api";
import type { ConflictHandler } from "@/shared/lib";
import { Button, ErrorText, Modal } from "@/shared/ui";

interface Props {
  entryId: string;
  title: string;
  onDeleted: () => void;
  onConflict: ConflictHandler;
}

export function DeleteEntryButton({ entryId, title, onDeleted, onConflict }: Props) {
  const [open, setOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function confirm(force: boolean) {
    try {
      await deleteEntry(entryId, force);
      setOpen(false);
      onDeleted();
    } catch (e) {
      if (isApiError(e, "external_modification")) onConflict(() => confirm(true));
      else setError(errorMessage(e));
    }
  }

  return (
    <>
      <Button variant="danger" onClick={() => setOpen(true)}>
        Delete
      </Button>
      {open && (
        <Modal title="Delete entry" onClose={() => setOpen(false)}>
          <p>
            Delete “{title}”? This cannot be undone (apart from restoring a backup of the vault file).
          </p>
          <ErrorText message={error} />
          <div className="ui-modal__actions">
            <Button onClick={() => setOpen(false)}>Cancel</Button>
            <Button variant="danger" onClick={() => void confirm(false)}>
              Delete
            </Button>
          </div>
        </Modal>
      )}
    </>
  );
}
