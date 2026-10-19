import { AutoLockSelect } from "@/features/auto-lock";
import { ChangePasswordButton } from "@/features/change-master-password";
import { ExportButton } from "@/features/export-vault";
import { Button } from "@/shared/ui";

import "./toolbar.css";

export function VaultToolbar({ onLock }: { onLock: () => void }) {
  return (
    <header className="toolbar">
      <strong className="toolbar__brand">Burrow</strong>
      <span className="toolbar__spacer" />
      <AutoLockSelect />
      <ChangePasswordButton />
      <ExportButton />
      <Button onClick={onLock}>Lock</Button>
    </header>
  );
}
