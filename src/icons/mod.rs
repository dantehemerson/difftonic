//! Per-file nerd-font icon and color lookup for the title bar.
//!
//! Codepoints and colors are taken from LazyGit's icon set
//! (`pkg/commands/icons/icons.go`) for Nerd Fonts v3. LazyGit's mapping is
//! curated from `eza` and `nvim-web-devicons` and is the most consistent set
//! for downstream tooling (also matches `telescope.nvim`).
//!
//! Returns the default file icon and a neutral gray when nothing matches.

use std::path::Path;

const ICON_DEFAULT: &str = "\u{f15b}";
const COLOR_DEFAULT: u32 = 0x6b737d;

fn special_file_icon(name: &str) -> Option<&'static str> {
    Some(match name {
        "dockerfile" | "containerfile" | "dockerfile.dev" | "dockerfile.prod"
        | "compose.yaml" | "compose.yml"
        | "docker-compose.yaml" | "docker-compose.yml" => "\u{f0a8}",
        ".dockerignore" => "\u{f0a8}",
        "cargo.toml" | "cargo.lock" => "\u{e68b}",
        "package.json" => "\u{e68c}",
        "package-lock.json" | "pnpm-lock.yaml" | "pnpm-workspace.yaml"
        | "yarn.lock" | "bun.lock" | "bun.lockb" => "\u{f487}",
        "tsconfig.json" | "jsconfig.json" => "\u{e628}",
        "makefile" | "gnumakefile" => "\u{e779}",
        ".gitignore" | ".gitattributes" | ".gitmodules" | ".gitkeep" => "\u{f1d3}",
        ".editorconfig" => "\u{e615}",
        ".eslintrc" | ".eslintrc.json" | ".eslintrc.js" | ".eslintrc.cjs" | ".eslintrc.yml" => "\u{e60c}",
        ".prettierrc" | ".prettierrc.json" | ".prettierrc.js" | ".prettierrc.toml" => "\u{e6b2}",
        ".babelrc" | ".babelrc.json" => "\u{e60c}",
        "readme.md" | "readme" | "readme.txt" | "readme.rst" => "\u{e7c8}",
        "license" | "license.md" | "license.txt" | "licence" | "licence.md" | "licence.txt"
        | "copying" | "copying.md" => "\u{e60a}",
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
        "ts" | "mts" => "\u{f06e6}",
        "cts" => "\u{e628}",
        "tsx" => "\u{ed46}",
        "js" | "mjs" => "\u{f031e}",
        "jsx" => "\u{ed46}",
        "cjs" => "\u{f031e}",
        "rs" => "\u{e68b}",
        "py" | "pyi" | "pyc" => "\u{ed1b}",
        "go" => "\u{e627}",
        "java" => "\u{e738}",
        "kt" | "kts" => "\u{e634}",
        "swift" => "\u{e755}",
        "rb" => "\u{e791}",
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
        "c" => "\u{e61e}",
        "h" => "\u{f0fd}",
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" => "\u{e61d}",
        "cs" => "\u{f81a}",
        "fs" | "fsx" => "\u{e7a7}",
        "ml" => "\u{e7a7}",
        "zig" => "\u{e6a9}",
        "nim" => "\u{e677}",
        "d" => "\u{e7af}",
        "r" => "\u{f1c0}",
        "jl" => "\u{e624}",
        "sql" => "\u{e706}",
        "graphql" | "gql" => "\u{e662}",

        // Web / markup / styling
        "html" | "htm" | "xhtml" => "\u{e736}",
        "css" => "\u{e6b8}",
        "scss" | "sass" => "\u{e603}",
        "less" => "\u{e758}",
        "xml" => "\u{e619}",
        "vue" => "\u{e6a0}",
        "svelte" => "\u{e697}",
        "astro" => "\u{e6b3}",
        "haml" | "pug" | "jade" => "\u{e6b1}",
        "md" | "mdx" => "\u{f48a}",
        "tex" => "\u{e600}",
        "bib" => "\u{e601}",

        // Data / config
        "json" | "jsonc" | "json5" => "\u{e60b}",
        "yaml" | "yml" => "\u{e8eb}",
        "toml" => "\u{e6b2}",
        "ini" | "cfg" | "conf" => "\u{e615}",
        "csv" | "tsv" => "\u{f1c0}",
        "proto" => "\u{e6b2}",
        "graphqls" | "graphqlconf" => "\u{e662}",

        // Shell / scripting
        "sh" | "bash" | "zsh" | "fish" => "\u{e795}",
        "ps1" => "\u{e795}",
        "bat" | "cmd" => "\u{e795}",

        // Media
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif" => "\u{f03e}",
        "svg" => "\u{f1c7}",
        "pdf" => "\u{f1c1}",
        "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" => "\u{f03d}",
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "wma" | "opus" => "\u{f001}",

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
        "lock" => "\u{f487}",

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

fn special_file_color(name: &str) -> Option<u32> {
    Some(match name {
        "dockerfile" | "containerfile" | "dockerfile.dev" | "dockerfile.prod"
        | "compose.yaml" | "compose.yml"
        | "docker-compose.yaml" | "docker-compose.yml" => 0x458ee6,
        ".dockerignore" => 0x458ee6,
        "cargo.toml" | "cargo.lock" => 0xdea584,
        "package.json" => 0xe8274b,
        "package-lock.json" | "pnpm-lock.yaml" | "pnpm-workspace.yaml"
        | "yarn.lock" | "bun.lock" | "bun.lockb" => 0xfb923c,
        "tsconfig.json" | "jsconfig.json" => 0x519aba,
        "makefile" | "gnumakefile" => 0x6d8086,
        ".gitignore" | ".gitattributes" | ".gitmodules" | ".gitkeep" => 0xf54d27,
        ".editorconfig" => 0xcbcb41,
        ".eslintrc" | ".eslintrc.json" | ".eslintrc.js" | ".eslintrc.cjs" | ".eslintrc.yml" => 0xcbcb41,
        ".prettierrc" | ".prettierrc.json" | ".prettierrc.js" | ".prettierrc.toml" => 0xcbcb41,
        ".babelrc" | ".babelrc.json" => 0xcbcb41,
        "readme.md" | "readme" | "readme.txt" | "readme.rst" => 0x42b883,
        "license" | "license.md" | "license.txt" | "licence" | "licence.md" | "licence.txt"
        | "copying" | "copying.md" => 0xcbcb41,
        "todo.md" | "todos.md" => 0xcbcb41,
        "favicon.ico" => 0xcbcb41,
        "robots.txt" => 0xcbcb41,
        ".env" | ".envrc" => 0xcbcb41,
        _ => return None,
    })
}

fn extension_color(ext: &str) -> Option<u32> {
    Some(match ext {
        // Code
        "ts" | "mts" => 0x0188d1,
        "cts" => 0x519aba,
        "tsx" => 0x04bcd4,
        "js" | "mjs" | "cjs" => 0xffca29,
        "jsx" => 0xffca29,
        "rs" => 0xdea584,
        "py" | "pyi" | "pyc" => 0xfed836,
        "go" => 0x02acc1,
        "java" => 0xf89820,
        "kt" | "kts" => 0xa97bff,
        "swift" => 0xf05137,
        "rb" => 0xcc342d,
        "php" => 0x4f5d95,
        "lua" => 0x000080,
        "pl" | "pm" => 0x0298c3,
        "vim" => 0x199f4b,
        "el" | "elc" => 0x8a4a91,
        "clj" | "cljs" | "cljc" => 0x8dc149,
        "ex" | "exs" | "eex" => 0x6e3a8e,
        "erl" | "hrl" => 0xb83998,
        "hs" => 0x5e5086,
        "scala" | "sbt" => 0xdc322f,
        "dart" => 0x00b4ab,
        "c" => 0x599eff,
        "h" => 0xa074c4,
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" => 0xf34b7d,
        "cs" => 0x178600,
        "fs" | "fsx" => 0x30b9db,
        "ml" => 0xec8a8b,
        "zig" => 0xf7a41d,
        "nim" => 0xffe953,
        "d" => 0xb03931,
        "r" => 0x198ce7,
        "jl" => 0xa270ba,
        "sql" => 0xe38c00,
        "graphql" | "gql" => 0xe535ab,

        // Web / markup / styling
        "html" | "htm" | "xhtml" => 0xe44d26,
        "css" => 0x663399,
        "scss" | "sass" => 0xcb6079,
        "less" => 0x1d365d,
        "xml" => 0x0060ac,
        "vue" => 0x41b883,
        "svelte" => 0xff3e00,
        "astro" => 0xe23f67,
        "haml" | "pug" | "jade" => 0xcb6079,
        "md" | "mdx" => 0xdddddd,
        "tex" => 0x3f5f4f,
        "bib" => 0x6b7036,

        // Data / config
        "json" | "jsonc" | "json5" => 0xcbcb41,
        "yaml" | "yml" => 0xd70000,
        "toml" => 0x9c4221,
        "ini" | "cfg" | "conf" => 0xcbcb41,
        "csv" | "tsv" => 0x237f4d,
        "proto" => 0x9c4221,
        "graphqls" | "graphqlconf" => 0xe535ab,

        // Shell / scripting
        "sh" | "bash" | "zsh" | "fish" => 0x4d5a5e,
        "ps1" => 0x012456,
        "bat" | "cmd" => 0xcbcb41,

        // Media
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif" => 0xa074c4,
        "svg" => 0xfab81b,
        "pdf" => 0xb30b00,
        "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" => 0xb8a162,
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "wma" | "opus" => 0xfab81b,

        // Archives / binaries
        "zip" | "tar" | "gz" | "tgz" | "bz2" | "7z" | "rar" | "xz" | "zst" => 0xcbcb41,
        "iso" | "img" | "dmg" => 0xcbcb41,
        "exe" | "msi" | "app" => 0xcbcb41,
        "dll" | "so" | "dylib" => 0xcbcb41,
        "deb" | "rpm" => 0xcbcb41,
        "apk" | "aab" | "ipa" => 0xcbcb41,

        // Fonts
        "woff" | "woff2" | "ttf" | "otf" | "eot" => 0xcbcb41,

        // Text / log
        "txt" => 0xcbcb41,
        "log" => 0xcbcb41,
        "lock" => 0xfb923c,

        _ => return None,
    })
}

/// Return the Seti UI Theme color that should accompany the icon for the
/// given filename. Falls back to a neutral gray.
pub fn file_color(name: &str) -> u32 {
    let path = Path::new(name);
    let basename = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return COLOR_DEFAULT,
    };
    if let Some(color) = special_file_color(&basename.to_ascii_lowercase()) {
        return color;
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    extension_color(&ext.to_ascii_lowercase()).unwrap_or(COLOR_DEFAULT)
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
        assert_eq!(file_icon("Dockerfile"), "\u{f0a8}");
        assert_eq!(file_icon("dockerfile"), "\u{f0a8}");
        assert_eq!(file_icon("Cargo.toml"), "\u{e68b}");
        assert_eq!(file_icon("package.json"), "\u{e68c}");
        assert_eq!(file_icon("tsconfig.json"), "\u{e628}");
        assert_eq!(file_icon("Makefile"), "\u{e779}");
        assert_eq!(file_icon("README.md"), "\u{e7c8}");
        assert_eq!(file_icon("README"), "\u{e7c8}");
        assert_eq!(file_icon("LICENSE"), "\u{e60a}");
        assert_eq!(file_icon(".gitignore"), "\u{f1d3}");
        assert_eq!(file_icon("yarn.lock"), "\u{f487}");
    }

    #[test]
    fn extensions() {
        assert_eq!(file_icon("foo.ts"), "\u{f06e6}");
        assert_eq!(file_icon("foo.tsx"), "\u{ed46}");
        assert_eq!(file_icon("foo.js"), "\u{f031e}");
        assert_eq!(file_icon("foo.jsx"), "\u{ed46}");
        assert_eq!(file_icon("foo.rs"), "\u{e68b}");
        assert_eq!(file_icon("foo.py"), "\u{ed1b}");
        assert_eq!(file_icon("foo.go"), "\u{e627}");
        assert_eq!(file_icon("foo.json"), "\u{e60b}");
        assert_eq!(file_icon("foo.html"), "\u{e736}");
        assert_eq!(file_icon("foo.css"), "\u{e6b8}");
        assert_eq!(file_icon("foo.md"), "\u{f48a}");
        assert_eq!(file_icon("foo.mdx"), "\u{f48a}");
        assert_eq!(file_icon("foo.yml"), "\u{e8eb}");
        assert_eq!(file_icon("foo.yaml"), "\u{e8eb}");
        assert_eq!(file_icon("foo.toml"), "\u{e6b2}");
        assert_eq!(file_icon("foo.sh"), "\u{e795}");
        assert_eq!(file_icon("foo.bash"), "\u{e795}");
        assert_eq!(file_icon("foo.cpp"), "\u{e61d}");
        assert_eq!(file_icon("foo.c"), "\u{e61e}");
        assert_eq!(file_icon("foo.h"), "\u{f0fd}");
        assert_eq!(file_icon("foo.png"), "\u{f03e}");
        assert_eq!(file_icon("foo.svg"), "\u{f1c7}");
        assert_eq!(file_icon("foo.pdf"), "\u{f1c1}");
        assert_eq!(file_icon("foo.zip"), "\u{f187}");
        assert_eq!(file_icon("foo.lua"), "\u{e620}");
        assert_eq!(file_icon("foo.vue"), "\u{e6a0}");
        assert_eq!(file_icon("foo.svelte"), "\u{e697}");
        assert_eq!(file_icon("foo.swift"), "\u{e755}");
        assert_eq!(file_icon("foo.kt"), "\u{e634}");
        assert_eq!(file_icon("foo.sql"), "\u{e706}");
        assert_eq!(file_icon("foo.astro"), "\u{e6b3}");
    }

    #[test]
    fn paths_with_directories() {
        assert_eq!(file_icon("src/foo.ts"), "\u{f06e6}");
        assert_eq!(file_icon("packages/core/src/index.tsx"), "\u{ed46}");
    }

    #[test]
    fn case_insensitive_extension() {
        assert_eq!(file_icon("Foo.TS"), "\u{f06e6}");
        assert_eq!(file_icon("Foo.Rs"), "\u{e68b}");
    }

    #[test]
    fn case_insensitive_special_filename() {
        assert_eq!(file_icon("DOCKERFILE"), "\u{f0a8}");
        assert_eq!(file_icon("Makefile"), "\u{e779}");
    }

    #[test]
    fn default_color_for_unknown() {
        assert_eq!(file_color("xyz"), 0x6b737d);
        assert_eq!(file_color(""), 0x6b737d);
    }

    #[test]
    fn special_file_colors() {
        assert_eq!(file_color("Dockerfile"), 0x458ee6);
        assert_eq!(file_color("dockerfile"), 0x458ee6);
        assert_eq!(file_color("Cargo.toml"), 0xdea584);
        assert_eq!(file_color("package.json"), 0xe8274b);
        assert_eq!(file_color("package-lock.json"), 0xfb923c);
        assert_eq!(file_color("README.md"), 0x42b883);
        assert_eq!(file_color("LICENSE"), 0xcbcb41);
        assert_eq!(file_color(".gitignore"), 0xf54d27);
        assert_eq!(file_color(".dockerignore"), 0x458ee6);
        assert_eq!(file_color("yarn.lock"), 0xfb923c);
        assert_eq!(file_color("tsconfig.json"), 0x519aba);
        assert_eq!(file_color("Makefile"), 0x6d8086);
        assert_eq!(file_color(".editorconfig"), 0xcbcb41);
    }

    #[test]
    fn extension_colors() {
        assert_eq!(file_color("foo.ts"), 0x0188d1);
        assert_eq!(file_color("foo.tsx"), 0x04bcd4);
        assert_eq!(file_color("foo.js"), 0xffca29);
        assert_eq!(file_color("foo.jsx"), 0xffca29);
        assert_eq!(file_color("foo.rs"), 0xdea584);
        assert_eq!(file_color("foo.py"), 0xfed836);
        assert_eq!(file_color("foo.go"), 0x02acc1);
        assert_eq!(file_color("foo.json"), 0xcbcb41);
        assert_eq!(file_color("foo.html"), 0xe44d26);
        assert_eq!(file_color("foo.css"), 0x663399);
        assert_eq!(file_color("foo.md"), 0xdddddd);
        assert_eq!(file_color("foo.yml"), 0xd70000);
        assert_eq!(file_color("foo.toml"), 0x9c4221);
        assert_eq!(file_color("foo.sh"), 0x4d5a5e);
        assert_eq!(file_color("foo.bash"), 0x4d5a5e);
        assert_eq!(file_color("foo.c"), 0x599eff);
        assert_eq!(file_color("foo.h"), 0xa074c4);
        assert_eq!(file_color("foo.lua"), 0x000080);
        assert_eq!(file_color("foo.svg"), 0xfab81b);
        assert_eq!(file_color("foo.pdf"), 0xb30b00);
        assert_eq!(file_color("foo.astro"), 0xe23f67);
    }

    #[test]
    fn color_and_icon_match_per_extension() {
        assert_ne!(file_color("foo.ts"), file_color("foo.rs"));
        assert_ne!(file_color("foo.py"), file_color("foo.go"));
        assert_ne!(file_color("foo.json"), file_color("foo.yaml"));
    }
}
