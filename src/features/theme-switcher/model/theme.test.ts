import { afterEach, expect, test, vi } from "vitest";

import { applyTheme, getStoredTheme, initTheme, parseTheme, storeTheme } from "./theme";

afterEach(() => {
  localStorage.clear();
  document.documentElement.removeAttribute("data-theme");
  vi.restoreAllMocks();
});

test("parseTheme falls back to system for unknown values", () => {
  expect(parseTheme("dark")).toBe("dark");
  expect(parseTheme("light")).toBe("light");
  expect(parseTheme("purple")).toBe("system");
  expect(parseTheme(null)).toBe("system");
});

test("applyTheme sets data-theme for explicit choices and clears it for system", () => {
  applyTheme("dark");
  expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
  applyTheme("light");
  expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  applyTheme("system");
  expect(document.documentElement.hasAttribute("data-theme")).toBe(false);
});

test("the choice is persisted and restored", () => {
  storeTheme("dark");
  expect(getStoredTheme()).toBe("dark");
  expect(initTheme()).toBe("dark");
  expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
});

test("unavailable storage does not throw", () => {
  vi.spyOn(Storage.prototype, "getItem").mockImplementation(() => {
    throw new Error("blocked");
  });
  vi.spyOn(Storage.prototype, "setItem").mockImplementation(() => {
    throw new Error("blocked");
  });
  expect(getStoredTheme()).toBe("system");
  expect(() => storeTheme("light")).not.toThrow();
});
