# difftonic

Fast, syntax-highlighted Git diff renderer designed for use as a LazyGit `diffRenderer`.

Reads a `git diff` patch from stdin, parses it, tokenizes each code line via
tree-sitter, and emits an ANSI-colored, gutter-numbered unified diff to stdout.

## Features

- 🚀 **Lightning Fast** — Straight ANSI output with no async overhead
- 🔧 **LazyGit Integration** — Drop-in `diffRenderer` for LazyGit
- 🎨 **Syntax Highlighting** — Tree-sitter powered tokenization for 10+ languages
- 🌈 **Multiple Themes** — Dark, light, auto, system, and adaptive themes

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

### Homebrew (macOS)

```sh
brew install dantehemerson/tap/difftonic
```

### Cargo

```sh
cargo install difftonic
```

### Build locally

```sh
cargo build --release
# binary at target/release/difftonic
```

## Usage

```
difftonic [options] < patch
```

| Flag                    | Description                                                                                | Default              |
| ----------------------- | ------------------------------------------------------------------------------------------ | -------------------- |
| `--syntax-theme <id>`   | tree-sitter theme id for code highlighting.                                                | `github-dark-default`|
| `--theme <name>`        | Diff color theme: `dark`, `light`, `auto`, `system`, or `adaptive`.                          | `auto`               |
| `--no-line-numbers`     | Hide line number gutter.                                                                   | (shown)              |
| `--full`                | Highlight context lines too (default: changed lines only).                                 | off                  |
| `-w, --width <n>`       | Width for title bar and layout. Auto-detected from terminal.                               | terminal width       |
| `-h, --help`            | Show usage.                                                                                |                      |
| `-v, --version`         | Print version.                                                                             |                      |

### Examples

```sh
git diff --no-color | difftonic
git diff --no-color | difftonic --theme dark
git diff --no-color | difftonic --theme system
git diff --no-color | difftonic --theme adaptive
git diff --no-color | difftonic --no-line-numbers
git diff --no-color | difftonic -w 120
```

## Use with LazyGit

```yaml
# ~/.config/lazygit/config.yml (Linux)
# ~/Library/Application Support/lazygit/config.yml (macOS)
git:
  diffRenderers:
    - name: difftonic
      type: stdinFilter
      colorArg: never
      command: difftonic
```

LazyGit must produce uncolored diffs (`colorArg: never`) so the renderer can
parse the patch safely.

To pass theme options:

```yaml
git:
  diffRenderers:
    - name: difftonic-dark
      type: stdinFilter
      colorArg: never
      command: difftonic --theme dark
    - name: difftonic-light
      type: stdinFilter
      colorArg: never
      command: difftonic --theme light
```

Use the `|` keybinding inside LazyGit to cycle between renderers.

The `system` theme uses terminal-native ANSI colors for syntax and accents, so
the terminal's configured palette is preserved both directly and inside
LazyGit. It combines those colors with subtle built-in dark or light diff
backgrounds. The syntax theme selects the background variant, so use a light
syntax theme with a light terminal. The complete built-in dark or light theme
is only used when terminal coloring is unavailable.

The `adaptive` theme queries the terminal's foreground, background, and ANSI
palette, then derives muted surfaces and tinted diff backgrounds from those
colors. If palette queries are unavailable, including inside LazyGit, it falls
back to `system`.

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
