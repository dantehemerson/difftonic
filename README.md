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
      command: /absolute/path/to/diffview/bin/diffview
```

LazyGit must produce uncolored diffs (`colorArg: never`) so the renderer can
parse the patch safely.

The included `bin/diffview` shell wrapper invokes Bun with `src/cli.ts`. If you
prefer, you can symlink it into your `PATH`:

```sh
ln -s /absolute/path/to/diffview/bin/diffview ~/.local/bin/diffview
```

LazyGit runs inside its TUI and waits for the renderer process to exit, so the
command must not start a pager of its own.

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
