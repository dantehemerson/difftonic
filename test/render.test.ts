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
  test("emits title, hunk header, and code lines", async () => {
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
    expect(stripAnsi(out)).toContain("x.ts");
    expect(out).toContain("@@ -1,2 +1,2 @@");
    expect(stripAnsi(out)).toContain("const a = 1;");
    expect(stripAnsi(out)).toContain("const a = 2;");
    expect(out).not.toContain("diff --git");
    expect(out).not.toContain("--- a/");
    expect(out).not.toContain("+++ b/");
    expect(out).not.toContain("index ");
  });

  test("emits Hunk-style title with filename and stats", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,3 +1,3 @@
 keep
-old
+new
 end
`);
    const out = await renderPatch(parsePatch(text));
    const lines = out.split("\n");
    const titleLine = lines[1]!;
    const plain = stripAnsi(titleLine);
    expect(plain).toContain("x.ts");
    expect(plain).toContain("+1");
    expect(plain).toContain("-1");
    expect(out).toContain("\x1b[48;2;");
  });

  test("title for new files shows 'new' state label", async () => {
    const text = patch(`diff --git a/new.txt b/new.txt
new file mode 100644
index 0000000..abc1234
--- /dev/null
+++ b/new.txt
@@ -0,0 +1,2 @@
+hello
+world
`);
    const out = await renderPatch(parsePatch(text));
    const titleLine = stripAnsi(out.split("\n")[1]!);
    expect(titleLine).toContain("new");
    expect(titleLine).toContain("new.txt");
    expect(titleLine).toContain("+2");
    expect(titleLine).toContain("-0");
  });

  test("title for deleted files shows 'deleted' state label", async () => {
    const text = patch(`diff --git a/old.txt b/old.txt
deleted file mode 100644
index abc1234..0000000
--- a/old.txt
+++ /dev/null
@@ -1,2 +0,0 @@
-bye
-cruel world
`);
    const out = await renderPatch(parsePatch(text));
    const titleLine = stripAnsi(out.split("\n")[1]!);
    expect(titleLine).toContain("deleted");
    expect(titleLine).toContain("old.txt");
    expect(titleLine).toContain("-2");
  });

  test("title for renamed files shows 'renamed' state label", async () => {
    const text = patch(`diff --git a/old.txt b/new.txt
similarity index 100%
rename from old.txt
rename to new.txt
`);
    const out = await renderPatch(parsePatch(text));
    const titleLine = stripAnsi(out.split("\n")[1]!);
    expect(titleLine).toContain("renamed");
    expect(titleLine).toContain("old.txt");
    expect(titleLine).toContain("new.txt");
  });

  test("separator appears between files but not after the last one", async () => {
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
    const out = await renderPatch(parsePatch(text));
    const plain = stripAnsi(out);
    expect(plain).toContain("a.ts");
    expect(plain).toContain("b.ts");
    const separatorCount = (plain.match(/─{30,}/g) ?? []).length;
    expect(separatorCount).toBeGreaterThanOrEqual(1);
    const trailingLines = out.trimEnd().split("\n").slice(-3);
    expect(trailingLines.some((l) => /^─+$/.test(stripAnsi(l)))).toBe(false);
  });

  test("renders a colored rail to the left of every code line", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,3 +1,3 @@
 keep
-old
+new
 end
`);
    const out = await renderPatch(parsePatch(text));
    const plain = stripAnsi(out);
    const codeLines = plain
      .split("\n")
      .filter((l) => l.includes("│"));
    expect(codeLines.length).toBeGreaterThan(0);
    for (const line of codeLines) {
      expect(line.startsWith("▌")).toBe(true);
    }
    expect(out).toMatch(/\x1b\[38;2;\d+;\d+;\d+(;\d)?m▌/);
  });

  test("rail uses addition accent for additions and deletion accent for deletions", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text));
    const additionLine = stripAnsi(out).split("\n").find((l) => l.includes("+new"));
    const deletionLine = stripAnsi(out).split("\n").find((l) => l.includes("-old"));
    expect(additionLine).toBeTruthy();
    expect(deletionLine).toBeTruthy();
    expect(additionLine!.startsWith("▌")).toBe(true);
    expect(deletionLine!.startsWith("▌")).toBe(true);
    expect(out).toMatch(/\x1b\[38;2;\d+;\d+;\d+;1m▌/);
  });

  test("title block is three lines: empty bg, title, empty bg", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text), { titleWidth: 40 });
    const lines = out.split("\n");
    const titleBg = `48;2;43;49;56`;
    const padLineSgr = new RegExp(`\x1b\\[0m(?:\x1b\\[0m)*\x1b\\[${titleBg}m {40}`);
    expect(lines[0]).toMatch(padLineSgr);
    expect(lines[2]).toMatch(padLineSgr);
    const titlePlain = stripAnsi(lines[1]!);
    expect(titlePlain.length).toBe(40);
    expect(titlePlain).toContain("x.ts");
  });

  test("title fills the configured width", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text), { titleWidth: 50 });
    const lines = out.split("\n");
    expect(stripAnsi(lines[1]!).length).toBe(50);
    expect(stripAnsi(lines[0]!).length).toBe(50);
    expect(stripAnsi(lines[2]!).length).toBe(50);
  });

  test("title includes a Nerd Font file icon before the filename", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const out = await renderPatch(parsePatch(text), { titleWidth: 50 });
    const titlePlain = stripAnsi(out.split("\n")[1]!);
    expect(titlePlain).toContain("\u{F15B}");
    const iconIndex = titlePlain.indexOf("\u{F15B}");
    const xIndex = titlePlain.indexOf("x.ts");
    expect(iconIndex).toBeGreaterThanOrEqual(0);
    expect(xIndex).toBeGreaterThan(iconIndex);
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

  test("uses the configured syntax theme for token colors", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const defaultOut = await renderPatch(parsePatch(text), { syntaxTheme: "github-dark-default" });
    const altOut = await renderPatch(parsePatch(text), { syntaxTheme: "dracula" });
    expect(stripAnsi(defaultOut)).toBe(stripAnsi(altOut));
    expect(defaultOut).not.toBe(altOut);
  });

  test("uses the configured diff theme for backgrounds", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const darkOut = await renderPatch(parsePatch(text), { theme: undefined });
    const lightOut = await renderPatch(parsePatch(text), {
      syntaxTheme: "github-light-default",
    });
    expect(stripAnsi(darkOut)).toBe(stripAnsi(lightOut));
    expect(darkOut).not.toBe(lightOut);
  });

  test("hides line numbers when requested", async () => {
    const text = patch(`diff --git a/x.ts b/x.ts
index abc..def 100644
--- a/x.ts
+++ b/x.ts
@@ -1,1 +1,1 @@
-old
+new
`);
    const withNumbers = await renderPatch(parsePatch(text), { showLineNumbers: true });
    const withoutNumbers = await renderPatch(parsePatch(text), { showLineNumbers: false });
    expect(withNumbers).toContain("│");
    expect(withoutNumbers).not.toContain("│");
    expect(stripAnsi(withoutNumbers)).toContain("new");
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
