export const AVATAR_TONES = 6;

/** Stable tone (0..5) derived from the title, so an entry keeps its colour. */
export function avatarTone(title: string): number {
  let hash = 0;
  for (const ch of title.toLowerCase()) hash = (hash * 31 + ch.codePointAt(0)!) >>> 0;
  return hash % AVATAR_TONES;
}

export function avatarInitial(title: string): string {
  const first = [...title.trim()][0];
  return first ? first.toUpperCase() : "?";
}
