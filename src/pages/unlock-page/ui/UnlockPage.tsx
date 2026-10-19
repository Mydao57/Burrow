import { useState } from "react";

import { CreateVaultForm } from "@/features/create-vault";
import { UnlockForm } from "@/features/unlock-vault";
import type { VaultInfo } from "@/shared/api";
import { Button } from "@/shared/ui";

import "./unlock-page.css";

export function UnlockPage({ onOpened, notice }: { onOpened: (info: VaultInfo) => void; notice?: string | null }) {
  const [mode, setMode] = useState<"unlock" | "create">("unlock");

  return (
    <main className="unlock-page">
      <div className="unlock-page__card">
        <h1>Burrow</h1>
        <p className="muted">Local-only password manager</p>
        {notice && <p className="unlock-page__notice">{notice}</p>}
        {mode === "unlock" ? <UnlockForm onUnlocked={onOpened} /> : <CreateVaultForm onCreated={onOpened} />}
        <div className="spacer" />
        <Button variant="ghost" onClick={() => setMode(mode === "unlock" ? "create" : "unlock")}>
          {mode === "unlock" ? "Create a new vault" : "Open an existing vault"}
        </Button>
      </div>
    </main>
  );
}
