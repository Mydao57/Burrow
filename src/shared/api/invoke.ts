import { invoke } from "@tauri-apps/api/core";

import type { ApiErrorCode } from "./types";

export class ApiError extends Error {
  readonly code: ApiErrorCode;

  constructor(code: ApiErrorCode, message: string) {
    super(message);
    this.name = "ApiError";
    this.code = code;
  }
}

export function isApiError(e: unknown, code?: ApiErrorCode): e is ApiError {
  return e instanceof ApiError && (code === undefined || e.code === code);
}

export function errorMessage(e: unknown): string {
  return e instanceof Error ? e.message : "Something went wrong";
}

function toApiError(e: unknown): ApiError {
  if (typeof e === "object" && e !== null && "code" in e && "message" in e) {
    const { code, message } = e as { code: ApiErrorCode; message: string };
    return new ApiError(code, message);
  }
  return new ApiError("internal", typeof e === "string" ? e : "Unexpected error");
}

/** The single place in the front-end that talks to the Rust backend. */
export async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (e) {
    throw toApiError(e);
  }
}
