import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { throttle } from "./throttle";

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

test("runs at most once per interval", () => {
  const fn = vi.fn();
  const t = throttle(fn, 1000);
  t();
  t();
  expect(fn).toHaveBeenCalledTimes(1);
  vi.advanceTimersByTime(1000);
  t();
  expect(fn).toHaveBeenCalledTimes(2);
});
