import { describe, test, expect } from "bun:test";
import { languageForPath } from "../src/language";

describe("languageForPath", () => {
  test("detects TypeScript variants", () => {
    expect(languageForPath("foo.ts")).toBe("typescript");
    expect(languageForPath("foo.tsx")).toBe("tsx");
    expect(languageForPath("foo.mts")).toBe("typescript");
    expect(languageForPath("foo.cts")).toBe("typescript");
  });

  test("detects common languages", () => {
    expect(languageForPath("main.go")).toBe("go");
    expect(languageForPath("lib.rs")).toBe("rust");
    expect(languageForPath("app.py")).toBe("python");
    expect(languageForPath("server.js")).toBe("javascript");
  });

  test("detects config files", () => {
    expect(languageForPath("package.json")).toBe("json");
    expect(languageForPath(".eslintrc.json")).toBe("json");
  });

  test("handles paths with directories", () => {
    expect(languageForPath("src/components/Button.tsx")).toBe("tsx");
    expect(languageForPath("packages/foo/index.ts")).toBe("typescript");
  });

  test("falls back to text", () => {
    expect(languageForPath("README")).toBe("text");
  });

  test("returns text when path is undefined", () => {
    expect(languageForPath(undefined)).toBe("text");
  });

  test("recognizes special filenames", () => {
    expect(languageForPath("Dockerfile")).toBe("dockerfile");
    expect(languageForPath("Makefile")).toBe("makefile");
  });
});
