import { describe, test, expect } from "bun:test";
import { parsePatch } from "../src/patch/parse";
import { sanitize } from "../src/patch/sanitize";
import { renderPatch } from "../src/render";

function patch(s: string): string {
  return sanitize(s);
}

function stripAnsi(s: string): string {
  return s.replace(/\x1b\[[0-9;]*m/g, "");
}

function parseSgrCodes(sgr: string): Set<string> {
  const parts = sgr.split(";");
  const codes = new Set<string>();
  for (let i = 0; i < parts.length; i++) {
    const p = parts[i]!;
    if (p === "48" && parts[i + 1] === "2") {
      codes.add("48;2");
      i += 4;
    } else if (p === "38" && parts[i + 1] === "2") {
      codes.add("38;2");
      i += 4;
    } else {
      codes.add(p);
    }
  }
  return codes;
}

describe("sanitize", () => {
  test("removes ANSI escape codes", () => {
    const input = "diff --git a/x b/x\x1b[31m\x1b[m\n";
    expect(sanitize(input)).not.toContain("\x1b[");
  });
  test("normalizes CRLF", () => {
    expect(sanitize("a\r\nb\r\n")).toBe("a\nb\n");
  });
});

describe("parsePatch", () => {
  test("parses a simple modification", () => {
    const text = patch(`diff --git a/example.ts b/example.ts
index abc..def 100644
--- a/example.ts
+++ b/example.ts
@@ -1,3 +1,3 @@
 line one
-old
+new
 line three
`);
    const patches = parsePatch(text);
    expect(patches).toHaveLength(1);
    expect(patches[0]!.files).toHaveLength(1);
    expect(patches[0]!.files[0]!.name).toBe("example.ts");
    expect(patches[0]!.files[0]!.hunks).toHaveLength(1);
    expect(patches[0]!.files[0]!.hunks[0]!.hunkContent).toHaveLength(3);
  });

  test("parses new file", () => {
    const text = patch(`diff --git a/new.txt b/new.txt
new file mode 100644
index 0000000..abc1234
--- /dev/null
+++ b/new.txt
@@ -0,0 +1,2 @@
+hello
+world
`);
    const patches = parsePatch(text);
    expect(patches).toHaveLength(1);
    expect(patches[0]!.files[0]!.type).toBe("new");
  });

  test("parses deleted file", () => {
    const text = patch(`diff --git a/old.txt b/old.txt
deleted file mode 100644
index abc1234..0000000
--- a/old.txt
+++ /dev/null
@@ -1,2 +0,0 @@
-bye
-cruel world
`);
    const patches = parsePatch(text);
    expect(patches).toHaveLength(1);
    expect(patches[0]!.files[0]!.type).toBe("deleted");
  });

  test("parses renamed file", () => {
    const text = patch(`diff --git a/old.txt b/new.txt
similarity index 100%
rename from old.txt
rename to new.txt
`);
    const patches = parsePatch(text);
    expect(patches).toHaveLength(1);
    expect(patches[0]!.files[0]!.type).toBe("rename-pure");
    expect(patches[0]!.files[0]!.prevName).toBe("old.txt");
    expect(patches[0]!.files[0]!.name).toBe("new.txt");
  });

  test("parses multiple files", () => {
    const text = patch(`diff --git a/a.ts b/a.ts
index 111..222 100644
--- a/a.ts
+++ b/a.ts
@@ -1,1 +1,1 @@
-a
+b
diff --git a/b.ts b/b.ts
index 333..444 100644
--- a/b.ts
+++ b/b.ts
@@ -1,1 +1,1 @@
-c
+d
`);
    const patches = parsePatch(text);
    expect(patches[0]!.files).toHaveLength(2);
  });
});

describe("renderPatch", () => {
  test("emits file header, hunk header, and lines", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,2 +1,2 @@
-const a = 1;
+const a = 2;
 const b = 3;
`);
    const out = await renderPatch(parsePatch(text));
    expect(out).toContain("diff --git a/x.ts b/x.ts");
    expect(out).toContain("--- a/x.ts");
    expect(out).toContain("+++ b/x.ts");
    expect(out).toContain("@@ -1,2 +1,2 @@");
    expect(stripAnsi(out)).toContain("const a = 1;");
    expect(stripAnsi(out)).toContain("const a = 2;");
  });

  test("emits ANSI color codes", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text));
    expect(out).toContain("\x1b[");
    expect(out).toContain("\x1b[0m");
  });

  test("handles empty patch", async () => {
    const out = await renderPatch(parsePatch(""));
    expect(out.trim()).toBe("");
  });

  test("renders deletion marker with deletion background", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,0 @@
-removed
`);
    const out = await renderPatch(parsePatch(text));
    expect(stripAnsi(out)).toContain("removed");
    expect(out).toMatch(/\x1b\[48;2;\d+;\d+;\d+/);
  });

  test("preserves multi-line additions", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,2 +1,5 @@
 first
+second
+third
+fourth
 fifth
`);
    const out = await renderPatch(parsePatch(text));
    expect(stripAnsi(out)).toContain("second");
    expect(stripAnsi(out)).toContain("third");
    expect(stripAnsi(out)).toContain("fourth");
    const plusCount = (out.match(/\x1b\[48;2;\d+;\d+;\d+;38;2;\d+;\d+;\d+;1m\+/g) ?? []).length;
    expect(plusCount).toBeGreaterThanOrEqual(3);
  });

  test("context code is not dimmed", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,3 +1,3 @@
 keep me
-not dim
+not dim
 end
`);
    const out = await renderPatch(parsePatch(text));
    const lines = out.split("\n");
    const plainLines = lines.map(stripAnsi);
    const contextLineIndices = plainLines
      .map((l, i) => (/│\s+(keep me|end)/.test(l) ? i : -1))
      .filter((i) => i >= 0);
    expect(contextLineIndices.length).toBeGreaterThan(0);
    for (const idx of contextLineIndices) {
      const line = lines[idx]!;
      const sgrs = [...line.matchAll(/\x1b\[0m(?:\x1b\[0m)*\x1b\[([0-9;]*)m/g)].map((m) => m[1]!);
      const codeSgrs = sgrs.filter((sgr) => {
        const codes = parseSgrCodes(sgr);
        return !codes.has("2") && !codes.has("1");
      });
      expect(codeSgrs.length).toBeGreaterThan(0);
    }
    expect(stripAnsi(out)).toContain("keep me");
  });

  test("addition code carries only the addition background, not bold or accent fg", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text));
    const match = out.match(/\x1b\[0m\x1b\[([0-9;]*)m\+\x1b\[0m\x1b\[([0-9;]*)m/);
    expect(match).toBeTruthy();
    const markerSgr = match![1]!;
    const codeBaseSgr = match![2]!;
    const markerCodes = parseSgrCodes(markerSgr);
    expect(markerCodes.has("1")).toBe(true);
    expect(markerCodes.has("48;2")).toBe(true);
    expect(codeBaseSgr).toMatch(/^48;2;\d+;\d+;\d+;38;2;\d+;\d+;\d+$/);
    const codes = parseSgrCodes(codeBaseSgr);
    expect(codes.has("1")).toBe(false);
    expect(codes.has("2")).toBe(false);
    expect(codes.has("48;2")).toBe(true);
    expect(stripAnsi(out)).toContain("new");
  });
});
