import { useState } from "react";

import { EntryRow, useEntries } from "@/entities/entry";
import { AddEntryButton } from "@/features/add-entry";
import { PasswordGenerator } from "@/features/generate-password";
import { SearchBox } from "@/features/search-entries";
import type { ConflictHandler } from "@/shared/lib";

import "./entry-list.css";

interface Props {
  version: number;
  selectedId: string | null;
  onSelect: (id: string | null) => void;
  onChanged: () => void;
  onConflict: ConflictHandler;
}

export function EntryList({ version, selectedId, onSelect, onChanged, onConflict }: Props) {
  const [query, setQuery] = useState("");
  const { entries, error } = useEntries(query, version);

  return (
    <section className="entry-list" aria-label="Entries">
      <header className="entry-list__header">
        <SearchBox value={query} onChange={setQuery} />
        <AddEntryButton
          onConflict={onConflict}
          onSaved={(id) => {
            onChanged();
            onSelect(id);
          }}
          renderPasswordTools={(setPassword) => <PasswordGenerator onGenerate={setPassword} />}
        />
      </header>
      {error && <p className="ui-error">{error}</p>}
      <ul className="entry-list__items">
        {entries.map((entry) => (
          <EntryRow key={entry.id} entry={entry} selected={entry.id === selectedId} onSelect={onSelect} />
        ))}
      </ul>
      {entries.length === 0 && !error && (
        <p className="muted entry-list__empty">{query ? "No matching entries." : "No entries yet. Add your first one."}</p>
      )}
    </section>
  );
}
