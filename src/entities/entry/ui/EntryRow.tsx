import type { EntrySummary } from "@/shared/api";

import { EntryAvatar } from "./EntryAvatar";

import "./entry.css";

interface Props {
  entry: EntrySummary;
  selected: boolean;
  onSelect: (id: string) => void;
}

export function EntryRow({ entry, selected, onSelect }: Props) {
  return (
    <li>
      <button
        type="button"
        className={`entry-row${selected ? " entry-row--selected" : ""}`}
        onClick={() => onSelect(entry.id)}
        aria-current={selected}
      >
        <EntryAvatar title={entry.title} />
        <span className="entry-row__text">
          <span className="entry-row__title">{entry.title}</span>
          <span className="entry-row__sub">{entry.username || entry.url || "No username"}</span>
        </span>
      </button>
    </li>
  );
}
