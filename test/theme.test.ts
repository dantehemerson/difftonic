import { describe, test, expect } from "bun:test";
import {
  DARK_THEME,
  LIGHT_THEME,
  isLightSyntaxTheme,
  resolveDiffTheme,
} from "../src/theme";

describe("isLightSyntaxTheme", () => {
  test("returns true for known light theme ids", () => {
    expect(isLightSyntaxTheme("github-light-default")).toBe(true);
    expect(isLightSyntaxTheme("github-light-high-contrast")).toBe(true);
    expect(isLightSyntaxTheme("one-light")).toBe(true);
    expect(isLightSyntaxTheme("solarized-light")).toBe(true);
  });

  test("returns false for known dark theme ids", () => {
    expect(isLightSyntaxTheme("github-dark-default")).toBe(false);
    expect(isLightSyntaxTheme("dracula")).toBe(false);
    expect(isLightSyntaxTheme("monokai")).toBe(false);
    expect(isLightSyntaxTheme("nord")).toBe(false);
  });
});

describe("resolveDiffTheme", () => {
  test("returns dark when explicitly requested", () => {
    expect(resolveDiffTheme("dark", "github-light-default")).toBe(DARK_THEME);
    expect(resolveDiffTheme("dark", "dracula")).toBe(DARK_THEME);
  });

  test("returns light when explicitly requested", () => {
    expect(resolveDiffTheme("light", "github-dark-default")).toBe(LIGHT_THEME);
    expect(resolveDiffTheme("light", "monokai")).toBe(LIGHT_THEME);
  });

  test("returns light for light syntax themes in auto mode", () => {
    expect(resolveDiffTheme("auto", "github-light-default")).toBe(LIGHT_THEME);
    expect(resolveDiffTheme("auto", "one-light")).toBe(LIGHT_THEME);
  });

  test("returns dark for dark syntax themes in auto mode", () => {
    expect(resolveDiffTheme("auto", "github-dark-default")).toBe(DARK_THEME);
    expect(resolveDiffTheme("auto", "dracula")).toBe(DARK_THEME);
    expect(resolveDiffTheme("auto", "monokai")).toBe(DARK_THEME);
  });

  test("treats undefined as auto", () => {
    expect(resolveDiffTheme(undefined, "github-light-default")).toBe(LIGHT_THEME);
    expect(resolveDiffTheme(undefined, "github-dark-default")).toBe(DARK_THEME);
  });
});
