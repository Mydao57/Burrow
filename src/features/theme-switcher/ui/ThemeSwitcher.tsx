import { Icon, type IconName } from "@/shared/ui";

import { useTheme } from "../model/useTheme";
import type { ThemePreference } from "../model/theme";

import "./theme-switcher.css";

const OPTIONS: { value: ThemePreference; label: string; icon: IconName }[] = [
  { value: "light", label: "Light", icon: "sun" },
  { value: "system", label: "System", icon: "monitor" },
  { value: "dark", label: "Dark", icon: "moon" },
];

export function ThemeSwitcher() {
  const { theme, setTheme } = useTheme();

  return (
    <div className="theme-switcher" role="radiogroup" aria-label="Theme">
      {OPTIONS.map(({ value, label, icon }) => (
        <button
          key={value}
          type="button"
          role="radio"
          aria-checked={theme === value}
          aria-label={label}
          title={label}
          className={`theme-switcher__option${theme === value ? " theme-switcher__option--active" : ""}`}
          onClick={() => setTheme(value)}
        >
          <Icon name={icon} size={15} />
        </button>
      ))}
    </div>
  );
}
