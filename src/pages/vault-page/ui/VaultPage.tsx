import { useState } from "react";

import { useConflictResolver } from "@/features/resolve-conflict";
import { lockVault } from "@/shared/api";
import { EntryDetail } from "@/widgets/entry-detail";
import { EntryList } from "@/widgets/entry-list";
import { VaultToolbar } from "@/widgets/vault-toolbar";

import "./vault-page.css";

export function VaultPage({ onLocked }: { onLocked: () => void }) {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [version, setVersion] = useState(0);
  const [epoch, setEpoch] = useState(0); // bumped on reload to remount editors with fresh state
  const bump = () => setVersion((v) => v + 1);

  const { onConflict, dialog } = useConflictResolver(() => {
    setSelectedId(null);
    setEpoch((e) => e + 1);
    bump();
  });

  async function lock() {
    await lockVault();
    onLocked();
  }

  return (
    <div className="vault-page">
      <VaultToolbar onLock={() => void lock()} />
      <div className="vault-page__body" key={epoch}>
        <EntryList
          version={version}
          selectedId={selectedId}
          onSelect={setSelectedId}
          onChanged={bump}
          onConflict={onConflict}
        />
        <EntryDetail
          entryId={selectedId}
          version={version}
          onChanged={bump}
          onDeleted={() => {
            setSelectedId(null);
            bump();
          }}
          onConflict={onConflict}
        />
      </div>
      {dialog}
    </div>
  );
}
