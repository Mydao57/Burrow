export type ThemePreference = "system" | "light" | "dark";

const STORAGE_KEY = "burrow.theme";
const VALID: readonly ThemePreference[] = ["system", "light", "dark"];

export function parseTheme(value: unknown): ThemePreference {
  return VALID.includes(value as ThemePreference) ? (value as ThemePreference) : "system";
}

export function getStoredTheme(): ThemePreference {
  try {
    return parseTheme(localStorage.getItem(STORAGE_KEY));
  } catch {
    return "system"; // storage unavailable: follow the OS
  }
}

export function storeTheme(pref: ThemePreference): void {
  try {
    localStorage.setItem(STORAGE_KEY, pref);
  } catch {
    // Not persisted; the choice still applies for this session.
  }
}

/** "system" removes the attribute so the `prefers-color-scheme` media query decides. */
export function applyTheme(pref: ThemePreference, root: HTMLElement = document.documentElement): void {
  if (pref === "system") root.removeAttribute("data-theme");
  else root.setAttribute("data-theme", pref);
}

/** Call once before the first render to avoid a flash of the wrong theme. */
export function initTheme(): ThemePreference {
  const pref = getStoredTheme();
  applyTheme(pref);
  return pref;
}
