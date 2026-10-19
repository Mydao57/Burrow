import { useEffect, useState } from "react";

import { CopyButton, RevealPassword } from "@/features/copy-secret";
import { DeleteEntryButton } from "@/features/delete-entry";
import { EditEntryButton } from "@/features/edit-entry";
import { PasswordGenerator } from "@/features/generate-password";
import { errorMessage, getEntry, type EntryDetail as EntryDetailDto } from "@/shared/api";
import type { ConflictHandler } from "@/shared/lib";

import "./entry-detail.css";

interface Props {
  entryId: string | null;
  version: number;
  onChanged: () => void;
  onDeleted: () => void;
  onConflict: ConflictHandler;
}

export function EntryDetail({ entryId, version, onChanged, onDeleted, onConflict }: Props) {
  const [entry, setEntry] = useState<EntryDetailDto | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!entryId) return setEntry(null);
    let cancelled = false;
    getEntry(entryId)
      .then((e) => !cancelled && (setEntry(e), setError(null)))
      .catch((e) => !cancelled && (setEntry(null), setError(errorMessage(e))));
    return () => {
      cancelled = true;
    };
  }, [entryId, version]);

  if (error) return <p className="ui-error entry-detail">{error}</p>;
  if (!entry) return <p className="muted entry-detail">Select an entry.</p>;

  return (
    <article className="entry-detail">
      <header className="entry-detail__header">
        <h1>{entry.title}</h1>
        <div className="entry-detail__actions">
          <EditEntryButton
            entry={entry}
            onSaved={onChanged}
            onConflict={onConflict}
            renderPasswordTools={(setPassword) => <PasswordGenerator onGenerate={setPassword} />}
          />
          <DeleteEntryButton entryId={entry.id} title={entry.title} onDeleted={onDeleted} onConflict={onConflict} />
        </div>
      </header>

      <dl>
        <dt>Username</dt>
        <dd>
          <span>{entry.username || "—"}</span>
          {entry.username && <CopyButton entryId={entry.id} field="username" />}
        </dd>
        <dt>Password</dt>
        <dd>
          <RevealPassword entryId={entry.id} />
          <CopyButton entryId={entry.id} field="password" />
        </dd>
        <dt>URL</dt>
        <dd>{entry.url || "—"}</dd>
        <dt>Notes</dt>
        <dd className="entry-detail__notes">{entry.notes || "—"}</dd>
        <dt>Modified</dt>
        <dd>{new Date(entry.modified_at).toLocaleString()}</dd>
      </dl>
    </article>
  );
}
