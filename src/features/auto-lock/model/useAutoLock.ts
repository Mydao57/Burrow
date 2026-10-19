import { useEffect } from "react";

import { onVaultAutoLocked, touchActivity } from "@/shared/api";
import { throttle } from "@/shared/lib";

const ACTIVITY_EVENTS = ["pointerdown", "keydown", "wheel", "pointermove"] as const;
const PING_EVERY_MS = 5_000;

/**
 * While `active`, reports user activity to the backend (which owns the idle
 * timer) and calls `onLocked` when the backend locks the vault.
 */
export function useAutoLock(active: boolean, onLocked: () => void) {
  useEffect(() => {
    if (!active) return;

    const ping = throttle(() => void touchActivity().catch(() => undefined), PING_EVERY_MS);
    ACTIVITY_EVENTS.forEach((ev) => window.addEventListener(ev, ping, { passive: true }));

    let unlisten: (() => void) | undefined;
    let disposed = false;
    onVaultAutoLocked(() => onLocked()).then((fn) => (disposed ? fn() : (unlisten = fn)));

    return () => {
      disposed = true;
      unlisten?.();
      ACTIVITY_EVENTS.forEach((ev) => window.removeEventListener(ev, ping));
    };
  }, [active, onLocked]);
}
