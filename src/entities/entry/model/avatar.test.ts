import { expect, test } from "vitest";

import { AVATAR_TONES, avatarInitial, avatarTone } from "./avatar";

test("tone is stable, case-insensitive and in range", () => {
  expect(avatarTone("GitHub")).toBe(avatarTone("github"));
  for (const t of ["", "a", "Bank", "été", "🔐 vault"]) {
    const tone = avatarTone(t);
    expect(tone).toBeGreaterThanOrEqual(0);
    expect(tone).toBeLessThan(AVATAR_TONES);
  }
});

test("initial handles empty, accents and emoji", () => {
  expect(avatarInitial("github")).toBe("G");
  expect(avatarInitial("  été")).toBe("É");
  expect(avatarInitial("🔐 vault")).toBe("🔐");
  expect(avatarInitial("   ")).toBe("?");
});
