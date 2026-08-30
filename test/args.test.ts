import { describe, test, expect } from "bun:test";
import { parseArgs } from "../src/args";

describe("parseArgs", () => {
  test("returns defaults for empty input", () => {
    const args = parseArgs([]);
    expect(args.help).toBe(false);
    expect(args.version).toBe(false);
    expect(args.noLineNumbers).toBe(false);
    expect(args.theme).toBeUndefined();
    expect(args.syntaxTheme).toBeUndefined();
    expect(args.positional).toEqual([]);
  });

  test("parses --help and -h", () => {
    expect(parseArgs(["--help"]).help).toBe(true);
    expect(parseArgs(["-h"]).help).toBe(true);
  });

  test("parses --version and -v", () => {
    expect(parseArgs(["--version"]).version).toBe(true);
    expect(parseArgs(["-v"]).version).toBe(true);
  });

  test("parses --no-line-numbers", () => {
    expect(parseArgs(["--no-line-numbers"]).noLineNumbers).toBe(true);
  });

  test("parses --theme with valid values", () => {
    expect(parseArgs(["--theme", "dark"]).theme).toBe("dark");
    expect(parseArgs(["--theme", "light"]).theme).toBe("light");
    expect(parseArgs(["--theme", "auto"]).theme).toBe("auto");
  });

  test("rejects --theme with no value", () => {
    expect(() => parseArgs(["--theme"])).toThrow(/requires a value/);
    expect(() => parseArgs(["--theme", "--syntax-theme"])).toThrow(/requires a value/);
  });

  test("rejects --theme with invalid value", () => {
    expect(() => parseArgs(["--theme", "purple"])).toThrow(/must be one of/);
  });

  test("parses --syntax-theme", () => {
    expect(parseArgs(["--syntax-theme", "dracula"]).syntaxTheme).toBe("dracula");
  });

  test("accepts --shiki-theme as alias", () => {
    expect(parseArgs(["--shiki-theme", "monokai"]).syntaxTheme).toBe("monokai");
  });

  test("rejects --syntax-theme with no value", () => {
    expect(() => parseArgs(["--syntax-theme"])).toThrow(/requires a value/);
  });

  test("rejects unknown long flags", () => {
    expect(() => parseArgs(["--bogus"])).toThrow(/Unknown option/);
  });

  test("rejects unknown short flags", () => {
    expect(() => parseArgs(["-x"])).toThrow(/Unknown option/);
  });

  test("captures positional arguments", () => {
    expect(parseArgs(["foo", "bar"]).positional).toEqual(["foo", "bar"]);
  });

  test("mixes flags and positionals", () => {
    const args = parseArgs(["--syntax-theme", "dracula", "--no-line-numbers", "extra"]);
    expect(args.syntaxTheme).toBe("dracula");
    expect(args.noLineNumbers).toBe(true);
    expect(args.positional).toEqual(["extra"]);
  });

  test("handles multiple flags in any order", () => {
    const args = parseArgs([
      "--no-line-numbers",
      "--syntax-theme",
      "github-light-default",
      "--theme",
      "light",
    ]);
    expect(args.noLineNumbers).toBe(true);
    expect(args.syntaxTheme).toBe("github-light-default");
    expect(args.theme).toBe("light");
  });
});
