import { Icon } from "@/shared/ui";

import "./search.css";

interface Props {
  value: string;
  onChange: (value: string) => void;
}

export function SearchBox({ value, onChange }: Props) {
  return (
    <label className="search-box">
      <Icon name="search" size={15} />
      <input
        type="search"
        placeholder="Search…"
        aria-label="Search entries"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        autoComplete="off"
        spellCheck={false}
      />
    </label>
  );
}
