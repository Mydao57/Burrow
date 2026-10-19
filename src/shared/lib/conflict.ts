/**
 * Called by a feature when a write is refused because the vault file changed
 * on disk. `forceRetry` repeats the write and overwrites the file.
 */
export type ConflictHandler = (forceRetry: () => Promise<void>) => void;
