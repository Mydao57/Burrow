import { useEffect, useState } from "react";

import { listEntries, type EntrySummary } from "@/shared/api";

/** Loads the entry list for `query`; reloads when `version` changes. */
export function useEntries(query: string, version: number) {
  const [entries, setEntries] = useState<EntrySummary[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    listEntries(query)
      .then((list) => !cancelled && (setEntries(list), setError(null)))
      .catch((e: Error) => !cancelled && setError(e.message));
    return () => {
      cancelled = true;
    };
  }, [query, version]);

  return { entries, error };
}
