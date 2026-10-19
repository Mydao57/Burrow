import { avatarInitial, avatarTone } from "../model/avatar";

import "./entry.css";

export function EntryAvatar({ title, size = "md" }: { title: string; size?: "md" | "lg" }) {
  return (
    <span className={`entry-avatar entry-avatar--${size} entry-avatar--tone-${avatarTone(title)}`} aria-hidden="true">
      {avatarInitial(title)}
    </span>
  );
}
