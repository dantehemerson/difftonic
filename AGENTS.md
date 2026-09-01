# Difftonic

Fast syntax-highlighted terminal diff renderer for LazyGit, written in Rust.

## Quick Reference

```sh
cargo test                          # run all tests
cargo test --test render            # run render tests only
cargo test --test syntax            # run syntax tests only
cargo test --test render hunk_      # run hunk tests only
cargo build --release               # build release binary
cargo install --path .              # install to ~/.cargo/bin/difftonic
```

## Project Structure

```
src/
  lib.rs            # core: parsing, rendering, themes, hunk indicators
  main.rs           # CLI entry point (clap args, terminal width detection)
  highlight/        # tree-sitter highlight queries per language
    mod.rs
    queries.rs
  icons/            # nerd-font icons + colors for title bar
    mod.rs
tests/
  render.rs         # render/output integration tests (33 tests)
  syntax.rs         # syntax highlighting tests (18 tests)
```

## Architecture

- `parse_patch()` reads unified diff from stdin, returns `Vec<FileDiff>`
- `render_file()` builds the title bar, hunk headers with indicators, and code lines
- `render_line()` renders a single code line with gutter (rail + line numbers + marker)
- `SyntaxHighlighter` wraps tree-sitter for per-line token coloring
- Themes are defined as `Theme` const structs (DARK, LIGHT)

## Code Conventions

- No comments in code unless explicitly requested
- Keep changes minimal and focused
- Follow existing patterns (look at neighboring code before adding new patterns)
- All public functions are in `lib.rs`; `main.rs` is thin
- Tests use `strip_ansi()` helper to verify visible output
- The `paint()` function wraps text in ANSI escape codes

## Key Types

- `RenderOptions` — width, theme, syntax_theme, no_line_numbers, full
- `Theme` — all color values (header_bg, hunk_bg, add_bg, del_bg, etc.)
- `FileDiff` — file name, state, hunks
- `Hunk` — header string, old_start, new_start, lines
- `DiffLine` — kind (Context/Addition/Deletion/NoNewline), text

## Running the Binary

```sh
git diff --no-color | difftonic
git diff --no-color | difftonic --theme dark
git diff --no-color | difftonic --no-line-numbers
git diff --no-color | difftonic -w 120
```

## LazyGit Config

```yaml
git:
  diffRenderers:
    - name: difftonic
      type: stdinFilter
      colorArg: never
      command: difftonic
```
