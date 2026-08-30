# diffview

Syntax-highlighted Git diff renderer designed for use as a LazyGit `diffRenderer`.

Reads a `git diff` patch from stdin, parses it through Pierre diffs (`@pierre/diffs`),
tokenizes each code line via Shiki, and emits an ANSI-colored, gutter-numbered
unified diff to stdout.

## Why

LazyGit already produces a correct diff command. This tool is a small, focused
replacement for `delta`/`diff-so-fancy` whose only job is to render that diff
nicely in a terminal — with proper syntax highlighting that survives the
addition/deletion backgrounds.

Inspired by the architecture used in
[hunk](https://github.com/modem-dev/hunk), but without OpenTUI, interactive UI,
session brokers, or any of the agent-annotation machinery. Only the parsing +
Shiki + terminal-coloring slice is kept.

## Architecture

```
stdin (git diff text)
  -> sanitize ANSI/CRLF
  -> @pierre/diffs parsePatchFiles
  -> per-file language detection (filename -> Shiki grammar)
  -> per-line Shiki tokenization
  -> terminal rendering (gutter + marker + colored tokens)
  -> stdout
```

- **`@pierre/diffs`** is reused for patch parsing and highlighter bootstrap.
- **Shiki** (pulled in by Pierre) handles tokenization.
- **No OpenTUI, no React, no worker pool** — straight ANSI output.

## Usage

```
diffview [options] < patch
```

### Output Structure

Each file in the diff gets its own section with two parts:

1. **Title bar** — a one-line header with the file's path, change state
   (`new` / `deleted` / `renamed`), and additions/deletions counts. Rendered
   with the diff theme's `fileHeaderBg` so the file boundary is obvious at
   a glance.
2. **Hunks** — the syntax-highlighted, gutter-numbered code body.

Files are separated by a `─` rule. The rule is omitted after the last file
so trailing whitespace doesn't accumulate.

The raw `diff --git` / `index` / `---` / `+++` metadata is intentionally
omitted — the title bar already carries the path and change state, and the
hunks carry the rest.

```
 example.ts                                                       deleted +0 -5
@@ -1,5 +0,0 @@

   1     │ -export function add(a: number, b: number): number {
   2     │ -  return a + b;
   3     │ -}
   4     │ -
   5     │ -// TODO: handle negative numbers
────────────────────────────────────────────────────────────────────────────────

 main.go                                                                  +5 -0
@@ -5,4 +5,9 @@ import "fmt"
```



| Flag                    | Description                                                                                  | Default                |
| ----------------------- | -------------------------------------------------------------------------------------------- | ---------------------- |
| `--syntax-theme <id>`   | Shiki theme id for code highlighting. Any bundled Shiki theme works.                         | `github-dark-default`  |
| `--theme <name>`        | Diff color theme: `dark`, `light`, or `auto` (picks dark/light based on the syntax theme).   | `auto`                 |
| `--no-line-numbers`     | Hide line number gutter.                                                                     | (line numbers shown)   |
| `-h`, `--help`          | Show usage.                                                                                  |                        |
| `-v`, `--version`       | Print version.                                                                               |                        |

### Examples

```sh
git diff --no-color | diffview

git diff --no-color | diffview --syntax-theme dracula

git diff --no-color | diffview --syntax-theme monokai --theme dark

git diff --no-color | diffview --syntax-theme github-light-default

git diff --no-color | diffview --no-line-numbers
```

Some well-known Shiki theme ids: `github-dark-default`, `github-light-default`,
`monokai`, `dracula`, `nord`, `one-dark-pro`, `one-light`, `catppuccin-mocha`,
`catppuccin-latte`, `solarized-dark`, `solarized-light`, `ayu-dark`,
`vitesse-dark`, `vitesse-light`, `rose-pine`, `tokyo-night`.

## Use with LazyGit

Add a `bin` directory to your `PATH`, then point LazyGit at the script:

```yaml
# ~/.config/lazygit/config.yml (Linux)
# ~/Library/Application Support/lazygit/config.yml (macOS)
git:
  diffRenderers:
    - name: diffview
      type: stdinFilter
      colorArg: never
      command: /absolute/path/to/diff_for_lazygit/bin/diffview
```

LazyGit must produce uncolored diffs (`colorArg: never`) so the renderer can
parse the patch safely.

The included `bin/diffview` shell wrapper invokes Bun with `src/cli.ts` if
`bin/diffview-bin` (built via `bun run build`) isn't present. Building
the binary is recommended for snappier LazyGit navigation. If you prefer,
you can symlink the wrapper into your `PATH`:

```sh
ln -s /absolute/path/to/diff_for_lazygit/bin/diffview ~/.local/bin/diffview
```

LazyGit runs inside its TUI and waits for the renderer process to exit, so the
command must not start a pager of its own.

To pass theme options through LazyGit, extend the `command:` field:

```yaml
git:
  diffRenderers:
    - name: diffview-dark
      type: stdinFilter
      colorArg: never
      command: /absolute/path/to/diff_for_lazygit/bin/diffview --syntax-theme dracula
    - name: diffview-light
      type: stdinFilter
      colorArg: never
      command: /absolute/path/to/diff_for_lazygit/bin/diffview --syntax-theme github-light-default --theme auto
```

Use the `|` keybinding inside LazyGit to cycle between renderers.

## Local development

```sh
bun install
bun test
bun run src/cli.ts < path/to/some.patch
```

To smoke-test against a real repo:

```sh
git diff --no-color | bun run src/cli.ts
```

Try a different theme:

```sh
git diff --no-color | bun run src/cli.ts --syntax-theme dracula --theme dark
```

## Performance: build a single-file binary

LazyGit spawns the renderer on every diff view, so a precompiled binary
makes the *warm* path noticeably snappier than `bun run`:

```sh
bun run build
```

This produces `bin/diffview-bin` (a self-contained Bun executable).
The `bin/diffview` wrapper automatically uses it when present, falling
back to `bun run src/cli.ts` otherwise.

Benchmark on a 50-line addition / 200-line context patch (macOS arm64):

| Path | Time |
| --- | --- |
| `bun run src/cli.ts` (cold) | ~180 ms |
| `bin/diffview-bin` (cold, including page-in) | ~1.2 s |
| `bin/diffview-bin` (warm) | **~170 ms** |

The cold-start penalty is the OS paging in the 73 MB binary; subsequent
invocations are nearly identical to `bun run`, since the dominant cost
is Shiki's highlighter initialization. Tokenization is now batched per
file so the highlighter shares grammar state across lines, and the
process preloads the language for the file paths mentioned in the
patch header (`+++ b/...`) before parsing.

For LazyGit, point the `command:` field at the `bin/diffview` wrapper
so the binary is used when available:
