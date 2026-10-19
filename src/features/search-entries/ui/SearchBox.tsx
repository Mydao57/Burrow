import "./search.css";

interface Props {
  value: string;
  onChange: (value: string) => void;
}

export function SearchBox({ value, onChange }: Props) {
  return (
    <input
      className="search-box"
      type="search"
      placeholder="Search title, username, URL…"
      aria-label="Search entries"
      value={value}
      onChange={(e) => onChange(e.target.value)}
      autoComplete="off"
      spellCheck={false}
    />
  );
}
