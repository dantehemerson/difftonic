//! Per-file nerd-font icon lookup for the title bar.
//!
//! Codepoints come from the popular Nerd Fonts "Seti UI + Custom" icon pack
//! and the "Codicons" pack (used in the devicon.nvim / NerdTree mappings).
//! Returns the default file icon when the extension is unknown.

use std::path::Path;

const ICON_DEFAULT: &str = "\u{f15b}";

fn special_file_icon(name: &str) -> Option<&'static str> {
    Some(match name {
        "dockerfile" | "dockerfile.dev" | "dockerfile.prod" => "\u{f308}",
        "containerfile" => "\u{f308}",
        "cargo.toml" | "cargo.lock" => "\u{e7a8}",
        "package.json" => "\u{e71e}",
        "package-lock.json" | "pnpm-lock.yaml" | "pnpm-workspace.yaml" => "\u{f023}",
        "yarn.lock" | "bun.lock" | "bun.lockb" => "\u{f023}",
        "tsconfig.json" | "jsconfig.json" => "\u{e628}",
        "makefile" | "gnumakefile" => "\u{e673}",
        ".gitignore" | ".gitattributes" | ".gitmodules" | ".gitkeep" => "\u{e1d3}",
        ".dockerignore" => "\u{f308}",
        ".editorconfig" => "\u{e652}",
        ".eslintrc" | ".eslintrc.json" | ".eslintrc.js" | ".eslintrc.cjs" | ".eslintrc.yml" => "\u{e74e}",
        ".prettierrc" | ".prettierrc.json" | ".prettierrc.js" | ".prettierrc.toml" => "\u{e6b2}",
        ".babelrc" | ".babelrc.json" => "\u{e74e}",
        "readme.md" | "readme" | "readme.txt" | "readme.rst" => "\u{e609}",
        "license" | "license.md" | "license.txt" | "licence" | "licence.md" | "licence.txt" => "\u{e60a}",
        "copying" | "copying.md" => "\u{e60a}",
        "todo.md" | "todos.md" => "\u{f046}",
        "favicon.ico" => "\u{f03e}",
        "robots.txt" => "\u{f15c}",
        ".env" | ".envrc" => "\u{f462}",
        _ => return None,
    })
}

fn extension_icon(ext: &str) -> Option<&'static str> {
    Some(match ext {
        // Code
        "ts" | "mts" | "cts" => "\u{e628}",
        "tsx" => "\u{e7ba}",
        "js" | "mjs" | "cjs" => "\u{e74e}",
        "jsx" => "\u{e7ba}",
        "rs" => "\u{e7a8}",
        "py" | "pyi" | "pyc" => "\u{e73c}",
        "go" => "\u{e626}",
        "java" => "\u{e738}",
        "kt" | "kts" => "\u{e634}",
        "swift" => "\u{e755}",
        "rb" => "\u{e739}",
        "php" => "\u{e73d}",
        "lua" => "\u{e620}",
        "pl" | "pm" => "\u{e67e}",
        "vim" => "\u{e62b}",
        "el" | "elc" => "\u{e779}",
        "clj" | "cljs" | "cljc" => "\u{e76a}",
        "ex" | "exs" | "eex" => "\u{e62d}",
        "erl" | "hrl" => "\u{e7b1}",
        "hs" => "\u{e777}",
        "scala" | "sbt" => "\u{e737}",
        "dart" => "\u{e798}",
        "c" | "h" => "\u{e61e}",
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" => "\u{e61d}",
        "cs" => "\u{f81a}",
        "fs" | "fsx" => "\u{e7a7}",
        "ml" => "\u{e7a7}",
        "zig" => "\u{f0e7}",
        "nim" => "\u{e677}",
        "d" => "\u{e7af}",
        "r" => "\u{f1c0}",
        "jl" => "\u{e624}",
        "sql" => "\u{e706}",
        "graphql" | "gql" => "\u{e662}",

        // Web / markup / styling
        "html" | "htm" | "xhtml" => "\u{e736}",
        "css" => "\u{e749}",
        "scss" | "sass" => "\u{e603}",
        "less" => "\u{e758}",
        "xml" => "\u{e619}",
        "vue" => "\u{f084}",
        "svelte" => "\u{e7a9}",
        "astro" => "\u{e7b1}",
        "haml" | "pug" | "jade" => "\u{e6b1}",
        "md" | "mdx" => "\u{e609}",
        "tex" => "\u{e600}",
        "bib" => "\u{e601}",

        // Data / config
        "json" | "jsonc" | "json5" => "\u{e60b}",
        "yaml" | "yml" => "\u{e6a8}",
        "toml" => "\u{e6b2}",
        "ini" | "cfg" | "conf" => "\u{e615}",
        "csv" | "tsv" => "\u{f1c0}",
        "proto" => "\u{e6b2}",
        "graphqls" | "graphqlconf" => "\u{e662}",

        // Shell / scripting
        "sh" | "bash" => "\u{f489}",
        "zsh" => "\u{f489}",
        "fish" => "\u{f489}",
        "ps1" => "\u{f489}",
        "bat" | "cmd" => "\u{f489}",

        // Media
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif" => "\u{f03e}",
        "svg" => "\u{f1c7}",
        "pdf" => "\u{f1c1}",
        "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" => "\u{f03d}",
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => "\u{f001}",
        "wma" | "opus" => "\u{f001}",

        // Archives / binaries
        "zip" | "tar" | "gz" | "tgz" | "bz2" | "7z" | "rar" | "xz" | "zst" => "\u{f187}",
        "iso" | "img" | "dmg" => "\u{f187}",
        "exe" | "msi" | "app" => "\u{f135}",
        "dll" | "so" | "dylib" => "\u{f135}",
        "deb" | "rpm" => "\u{f187}",
        "apk" | "aab" | "ipa" => "\u{f135}",

        // Fonts
        "woff" | "woff2" | "ttf" | "otf" | "eot" => "\u{f031}",

        // Text / log
        "txt" => "\u{f15c}",
        "log" => "\u{f15c}",
        "lock" => "\u{f023}",

        _ => return None,
    })
}

/// Return the nerd-font codepoint character that should appear before the
/// filename in the title bar, picking based on extension and a few common
/// special filenames. Falls back to a generic file icon.
pub fn file_icon(name: &str) -> &'static str {
    let path = Path::new(name);
    let basename = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return ICON_DEFAULT,
    };
    if let Some(icon) = special_file_icon(&basename.to_ascii_lowercase()) {
        return icon;
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    extension_icon(&ext.to_ascii_lowercase()).unwrap_or(ICON_DEFAULT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_icon_for_unknown() {
        assert_eq!(file_icon("xyz"), ICON_DEFAULT);
        assert_eq!(file_icon(""), ICON_DEFAULT);
        assert_eq!(file_icon("Makefile.suffix"), ICON_DEFAULT);
    }

    #[test]
    fn special_filenames() {
        assert_eq!(file_icon("Dockerfile"), "\u{f308}");
        assert_eq!(file_icon("dockerfile"), "\u{f308}");
        assert_eq!(file_icon("Cargo.toml"), "\u{e7a8}");
        assert_eq!(file_icon("package.json"), "\u{e71e}");
        assert_eq!(file_icon("tsconfig.json"), "\u{e628}");
        assert_eq!(file_icon("Makefile"), "\u{e673}");
        assert_eq!(file_icon("README.md"), "\u{e609}");
        assert_eq!(file_icon("README"), "\u{e609}");
        assert_eq!(file_icon("LICENSE"), "\u{e60a}");
        assert_eq!(file_icon(".gitignore"), "\u{e1d3}");
        assert_eq!(file_icon("yarn.lock"), "\u{f023}");
    }

    #[test]
    fn extensions() {
        assert_eq!(file_icon("foo.ts"), "\u{e628}");
        assert_eq!(file_icon("foo.tsx"), "\u{e7ba}");
        assert_eq!(file_icon("foo.js"), "\u{e74e}");
        assert_eq!(file_icon("foo.jsx"), "\u{e7ba}");
        assert_eq!(file_icon("foo.rs"), "\u{e7a8}");
        assert_eq!(file_icon("foo.py"), "\u{e73c}");
        assert_eq!(file_icon("foo.go"), "\u{e626}");
        assert_eq!(file_icon("foo.json"), "\u{e60b}");
        assert_eq!(file_icon("foo.html"), "\u{e736}");
        assert_eq!(file_icon("foo.css"), "\u{e749}");
        assert_eq!(file_icon("foo.md"), "\u{e609}");
        assert_eq!(file_icon("foo.mdx"), "\u{e609}");
        assert_eq!(file_icon("foo.yml"), "\u{e6a8}");
        assert_eq!(file_icon("foo.yaml"), "\u{e6a8}");
        assert_eq!(file_icon("foo.toml"), "\u{e6b2}");
        assert_eq!(file_icon("foo.sh"), "\u{f489}");
        assert_eq!(file_icon("foo.bash"), "\u{f489}");
        assert_eq!(file_icon("foo.cpp"), "\u{e61d}");
        assert_eq!(file_icon("foo.c"), "\u{e61e}");
        assert_eq!(file_icon("foo.h"), "\u{e61e}");
        assert_eq!(file_icon("foo.png"), "\u{f03e}");
        assert_eq!(file_icon("foo.svg"), "\u{f1c7}");
        assert_eq!(file_icon("foo.pdf"), "\u{f1c1}");
        assert_eq!(file_icon("foo.zip"), "\u{f187}");
        assert_eq!(file_icon("foo.lua"), "\u{e620}");
        assert_eq!(file_icon("foo.vue"), "\u{f084}");
        assert_eq!(file_icon("foo.svelte"), "\u{e7a9}");
        assert_eq!(file_icon("foo.swift"), "\u{e755}");
        assert_eq!(file_icon("foo.kt"), "\u{e634}");
        assert_eq!(file_icon("foo.sql"), "\u{e706}");
    }

    #[test]
    fn paths_with_directories() {
        assert_eq!(file_icon("src/foo.ts"), "\u{e628}");
        assert_eq!(file_icon("packages/core/src/index.tsx"), "\u{e7ba}");
    }

    #[test]
    fn case_insensitive_extension() {
        assert_eq!(file_icon("Foo.TS"), "\u{e628}");
        assert_eq!(file_icon("Foo.Rs"), "\u{e7a8}");
    }

    #[test]
    fn case_insensitive_special_filename() {
        assert_eq!(file_icon("DOCKERFILE"), "\u{f308}");
        assert_eq!(file_icon("Makefile"), "\u{e673}");
    }
}