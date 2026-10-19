import type { EntrySummary } from "@/shared/api";

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
        <span className="entry-row__title">{entry.title}</span>
        <span className="entry-row__sub">{entry.username || entry.url || "—"}</span>
      </button>
    </li>
  );
}
