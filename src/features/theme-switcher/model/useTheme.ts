import { useCallback, useState } from "react";

import { applyTheme, getStoredTheme, storeTheme, type ThemePreference } from "./theme";

export function useTheme() {
  const [theme, setThemeState] = useState<ThemePreference>(getStoredTheme);

  const setTheme = useCallback((pref: ThemePreference) => {
    setThemeState(pref);
    storeTheme(pref);
    applyTheme(pref);
  }, []);

  return { theme, setTheme };
}
