import { useCallback, useEffect, useState } from "react";

import { useAutoLock } from "@/features/auto-lock";
import { UnlockPage } from "@/pages/unlock-page";
import { VaultPage } from "@/pages/vault-page";
import { vaultStatus } from "@/shared/api";

export function App() {
  const [unlocked, setUnlocked] = useState(false);
  const [ready, setReady] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);

  useEffect(() => {
    vaultStatus()
      .then((s) => setUnlocked(s.unlocked))
      .finally(() => setReady(true));
  }, []);

  const onAutoLocked = useCallback(() => {
    setUnlocked(false);
    setNotice("Locked after inactivity.");
  }, []);
  useAutoLock(unlocked, onAutoLocked);

  if (!ready) return null;

  return unlocked ? (
    <VaultPage onLocked={() => setUnlocked(false)} />
  ) : (
    <UnlockPage
      notice={notice}
      onOpened={() => {
        setNotice(null);
        setUnlocked(true);
      }}
    />
  );
}
