import { AutoLockSelect } from "@/features/auto-lock";
import { ExportButton } from "@/features/export-vault";
import { ThemeSwitcher } from "@/features/theme-switcher";
import { Button, Icon } from "@/shared/ui";

import "./toolbar.css";

export function VaultToolbar({ onLock }: { onLock: () => void }) {
  return (
    <header className="toolbar">
      <div className="toolbar__brand">
        <span className="toolbar__logo">
          <Icon name="shield" size={18} />
        </span>
        <strong>Burrow</strong>
      </div>
      <span className="toolbar__spacer" />
      <AutoLockSelect />
      <ExportButton />
      <ThemeSwitcher />
      <Button icon="lock" onClick={onLock}>
        Lock
      </Button>
    </header>
  );
}
