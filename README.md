# diffview

Fast, syntax-highlighted Git diff renderer designed for use as a LazyGit `diffRenderer`.

Reads a `git diff` patch from stdin, parses it, tokenizes each code line via
tree-sitter, and emits an ANSI-colored, gutter-numbered unified diff to stdout.

## Architecture

```
stdin (git diff text)
  -> parse unified diff
  -> per-file language detection (filename -> tree-sitter grammar)
  -> per-line tree-sitter tokenization
  -> terminal rendering (title bar + gutter + rail + colored tokens)
  -> stdout
```

- **tree-sitter** handles syntax highlighting.
- **No async, no worker pool** — straight ANSI output.

## Install

```sh
cargo install --path .
```

Or build locally:

```sh
cargo build --release
# binary at target/release/diffview
```

## Usage

```
diffview [options] < patch
```

| Flag                    | Description                                                                                | Default              |
| ----------------------- | ------------------------------------------------------------------------------------------ | -------------------- |
| `--syntax-theme <id>`   | tree-sitter theme id for code highlighting.                                                | `github-dark-default`|
| `--theme <name>`        | Diff color theme: `dark`, `light`, `auto`, or `system` (uses the terminal palette).          | `auto`               |
| `--no-line-numbers`     | Hide line number gutter.                                                                   | (shown)              |
| `--full`                | Highlight context lines too (default: changed lines only).                                 | off                  |
| `-w, --width <n>`       | Width for title bar and layout. Auto-detected from terminal.                               | terminal width       |
| `-h, --help`            | Show usage.                                                                                |                      |
| `-v, --version`         | Print version.                                                                             |                      |

### Examples

```sh
git diff --no-color | diffview
git diff --no-color | diffview --theme dark
git diff --no-color | diffview --theme system
git diff --no-color | diffview --no-line-numbers
git diff --no-color | diffview -w 120
```

## Use with LazyGit

```yaml
# ~/.config/lazygit/config.yml (Linux)
# ~/Library/Application Support/lazygit/config.yml (macOS)
git:
  diffRenderers:
    - name: diffview
      type: stdinFilter
      colorArg: never
      command: diffview
```

LazyGit must produce uncolored diffs (`colorArg: never`) so the renderer can
parse the patch safely.

To pass theme options:

```yaml
git:
  diffRenderers:
    - name: diffview-dark
      type: stdinFilter
      colorArg: never
      command: diffview --theme dark
    - name: diffview-light
      type: stdinFilter
      colorArg: never
      command: diffview --theme light
```

Use the `|` keybinding inside LazyGit to cycle between renderers.

The `system` theme queries the controlling terminal for its foreground,
background, and ANSI palette when possible. It derives muted UI colors and
diff backgrounds from those values. When exact palette queries are unavailable,
including in LazyGit's renderer PTY, it combines terminal-native ANSI colors
with subtle built-in dark or light diff backgrounds. The syntax theme selects
the fallback variant, so use a light syntax theme with a light terminal. The
complete built-in dark or light theme is only used when terminal coloring is
unavailable.

## Output Structure

Each file in the diff gets its own section:

1. **Title bar** — full-width header with file icon, path, change state
   (`new` / `deleted` / `renamed`), and +/- counts.
2. **Hunk headers** — indented and aligned with source text, with direction
   indicators (`↑` `↓` `󰹹`) showing hidden context.
3. **Code body** — syntax-highlighted, gutter-numbered lines with a colored
   rail marking additions and deletions.

Files are separated by a `─` rule.

## Supported Languages

TypeScript, JavaScript, Rust, Python, Go, JSON, CSS, HTML, Bash, Markdown.

## Development

```sh
cargo test
cargo build --release
```

## License

MIT
