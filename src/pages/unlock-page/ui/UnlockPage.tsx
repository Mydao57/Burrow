import { useState } from "react";

import { CreateVaultForm } from "@/features/create-vault";
import { ThemeSwitcher } from "@/features/theme-switcher";
import { UnlockForm } from "@/features/unlock-vault";
import type { VaultInfo } from "@/shared/api";
import { Button, Icon, Logo } from "@/shared/ui";

import "./unlock-page.css";

export function UnlockPage({ onOpened, notice }: { onOpened: (info: VaultInfo) => void; notice?: string | null }) {
  const [mode, setMode] = useState<"unlock" | "create">("unlock");

  return (
    <main className="unlock-page">
      <div className="unlock-page__theme">
        <ThemeSwitcher />
      </div>
      <div className="unlock-page__card">
        <div className="unlock-page__brand">
          <Logo size={34} />
          <h1>Burrow</h1>
        </div>
        {notice && (
          <p className="unlock-page__notice">
            <Icon name="lock" size={14} /> {notice}
          </p>
        )}
        {mode === "unlock" ? <UnlockForm onUnlocked={onOpened} /> : <CreateVaultForm onCreated={onOpened} />}
        <div className="spacer" />
        <Button variant="ghost" size="sm" onClick={() => setMode(mode === "unlock" ? "create" : "unlock")}>
          {mode === "unlock" ? "Create a new vault" : "Open an existing vault"}
        </Button>
      </div>
    </main>
  );
}
