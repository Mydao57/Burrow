import { useEffect, useState } from "react";

import { setAutoLockMinutes, vaultStatus } from "@/shared/api";

const CHOICES = [1, 5, 15, 30, 60];

export function AutoLockSelect() {
  const [minutes, setMinutes] = useState<number | null>(null);

  useEffect(() => {
    vaultStatus().then((s) => setMinutes(s.auto_lock_minutes)).catch(() => undefined);
  }, []);

  if (minutes === null) return null;
  const choices = CHOICES.includes(minutes) ? CHOICES : [...CHOICES, minutes].sort((a, b) => a - b);

  return (
    <label className="muted">
      Auto-lock{" "}
      <select
        value={minutes}
        onChange={(e) => {
          const next = Number(e.target.value);
          setMinutes(next);
          void setAutoLockMinutes(next);
        }}
      >
        {choices.map((m) => (
          <option key={m} value={m}>
            {m} min
          </option>
        ))}
      </select>
    </label>
  );
}
