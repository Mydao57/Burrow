import { AutoLockSelect } from "@/features/auto-lock";
import { ExportButton } from "@/features/export-vault";
import { ThemeSwitcher } from "@/features/theme-switcher";
import { Button, Logo } from "@/shared/ui";

import "./toolbar.css";

export function VaultToolbar({ onLock }: { onLock: () => void }) {
  return (
    <header className="toolbar">
      <div className="toolbar__brand">
        <Logo size={22} />
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
