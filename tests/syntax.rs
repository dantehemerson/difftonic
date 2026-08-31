use diffview::{render, RenderOptions, Theme, DARK};

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(&next) = chars.peek() {
                chars.next();
                if next == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn sgr_fg(c: u32) -> String {
    format!("38;2;{};{};{}", (c >> 16) & 255, (c >> 8) & 255, c & 255)
}

fn color_for(category: &str) -> &'static str {
    let t = DARK;
    match category {
        "comment" => "127;132;142",
        "keyword" => "198;120;221",
        "string" => "152;195;121",
        "number" => "209;154;102",
        "type" => "229;192;123",
        "function" => "97;175;239",
        "punctuation" => "171;178;191",
        _ => panic!("unknown category: {category}"),
    }
    .to_string()
    .leak()
}

fn assert_has_category(output: &str, category: &str) {
    let expected = color_for(category);
    let found = output.contains(&format!("38;2;{}", expected))
        || output.contains(&expected)
        || output.contains(&format!("38;2;{};", expected));
    assert!(
        found,
        "expected SGR fg `{}` (for {}) in output, but it was missing",
        expected, category
    );
}

#[test]
fn typescript_changes_keep_keyword_color() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-const oldValue = 1;\n+const newValue = 2;\n";
    let output = render(input, &RenderOptions::default());
    let plain = strip_ansi(&output);
    assert!(plain.contains("-const oldValue = 1;"));
    assert!(plain.contains("+const newValue = 2;"));
    assert_has_category(&output, "keyword");
}

#[test]
fn typescript_changes_keep_string_color() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-const x: string = \"before\";\n+const x: string = \"after\";\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "string");
}

#[test]
fn typescript_changes_keep_number_color() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-const x: number = 1;\n+const x: number = 2;\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "number");
}

#[test]
fn typescript_changes_keep_comment_color() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-// old note\n+// new note\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "comment");
}

#[test]
fn typescript_changes_keep_type_color() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-let v: number = 1;\n+let v: number = 2;\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "type");
}

#[test]
fn rust_changes_keep_macro_color() {
    let input = "diff --git a/x.rs b/x.rs\nindex abc..def 100644\n--- a/x.rs\n+++ b/x.rs\n@@ -1,1 +1,1 @@\n-fn a() { println!(\"old\"); }\n+fn a() { println!(\"new\"); }\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "keyword");
    assert_has_category(&output, "string");
}

#[test]
fn multiline_jsdoc_highlight_preserved_in_additions() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,5 @@\n-// before\n+/**\n+ * Multiline JSDoc\n+ * spans multiple lines\n+ */\n function foo() {\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "comment");
}

#[test]
fn python_keyword_highlight_visible() {
    let input = "diff --git a/x.py b/x.py\nindex abc..def 100644\n--- a/x.py\n+++ b/x.py\n@@ -1,1 +1,1 @@\n-def hello(): return 1\n+def hello(): return 2\n";
    let output = render(input, &RenderOptions::default());
    assert_has_category(&output, "keyword");
    assert_has_category(&output, "number");
}

#[test]
fn syntax_colors_visible_on_dark_theme() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-const old = \"hello\";\n+const new = \"world\";\n";
    let output = render(input, &RenderOptions::default());
    let s = sgr_fg(DARK.syntax.string);
    let k = sgr_fg(DARK.syntax.keyword);
    assert!(output.contains(&s) || output.contains(&s.replace(';', "")));
    assert!(output.contains(&k) || output.contains(&k.replace(';', "")));
}

#[test]
fn highlight_indices_map_to_distinct_palette_colors() {
    let theme: Theme = DARK;
    assert_ne!(theme.syntax.comment, theme.syntax.keyword);
    assert_ne!(theme.syntax.keyword, theme.syntax.string);
    assert_ne!(theme.syntax.string, theme.syntax.number);
    assert_ne!(theme.syntax.number, theme.syntax.type_);
}

#[test]
fn gutter_uses_deletion_background_for_deletions() {
    let input = "diff --git a/x.txt b/x.txt\n--- a/x.txt\n+++ b/x.txt\n@@ -1,1 +1,1 @@\n-only_old\n+only_new\n";
    let output = render(input, &RenderOptions::default());
    let del_bg = format!(
        "48;2;{};{};{}",
        (DARK.del_gutter_bg >> 16) & 0xff,
        (DARK.del_gutter_bg >> 8) & 0xff,
        DARK.del_gutter_bg & 0xff
    );
    let add_bg = format!(
        "48;2;{};{};{}",
        (DARK.add_gutter_bg >> 16) & 0xff,
        (DARK.add_gutter_bg >> 8) & 0xff,
        DARK.add_gutter_bg & 0xff
    );
    let plain = strip_ansi(&output);
    let only_old_line = plain
        .split('\n')
        .find(|l| l.contains("-only_old"))
        .expect("deletion line");
    let only_new_line = plain
        .split('\n')
        .find(|l| l.contains("+only_new"))
        .expect("addition line");
    // The deletion line's gutter region should have the del_bg applied.
    let raw_only_old = output
        .split('\n')
        .find(|l| l.contains("only_old"))
        .unwrap_or_else(|| panic!("no deletion line in raw output"));
    assert!(
        raw_only_old.contains(&del_bg),
        "expected deletion bg {} in gutter, line={}",
        del_bg,
        raw_only_old
    );
    // The addition line's gutter region should have the add_bg applied.
    let raw_only_new = output
        .split('\n')
        .find(|l| l.contains("only_new"))
        .unwrap_or_else(|| panic!("no addition line in raw output"));
    assert!(
        raw_only_new.contains(&add_bg),
        "expected addition bg {} in gutter, line={}",
        add_bg,
        raw_only_new
    );
    // Sanity: make sure we did pick the right lines.
    assert!(only_old_line.starts_with('▌'));
    assert!(only_new_line.starts_with('▌'));
    let _ = (only_old_line, only_new_line);
}

#[test]
fn full_file_hides_hunk_header() {
    // Single hunk at line 1 covering the whole file: header is redundant.
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,4 @@\n a\n b\n+new\n c\n";
    let out = render(input, &RenderOptions::default());
    assert!(
        !out.contains("@@"),
        "hunk header should be suppressed for full-file diffs"
    );
}

#[test]
fn multi_hunk_keeps_hunk_headers() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,4 @@\n a\n b\n+inserted\n c\n@@ -10,3 +11,4 @@\n x\n y\n+z\n w\n";
    let out = render(input, &RenderOptions::default());
    assert!(out.contains("@@ -1,3 +1,4 @@"));
    assert!(out.contains("@@ -10,3 +11,4 @@"));
}

#[test]
fn single_hunk_not_at_line_one_keeps_header() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -50,3 +50,4 @@\n a\n b\n+inserted\n c\n";
    let out = render(input, &RenderOptions::default());
    assert!(out.contains("@@ -50,3 +50,4 @@"));
}

#[test]
fn hunk_header_hidden_for_context_only_hunk() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,11 +1,11 @@\n a\n b\n c\n d\n e\n f\n g\n h\n i\n j\n k\n";
    let out = render(input, &RenderOptions::default());
    assert!(
        !out.contains("@@"),
        "context-only hunk header should be hidden, got: {}",
        out
    );
}

#[test]
fn hunk_header_kept_for_hunk_with_changes() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -50,3 +50,4 @@\n a\n b\n+inserted\n c\n";
    let out = render(input, &RenderOptions::default());
    assert!(out.contains("@@ -50,3 +50,4 @@"));
}

#[test]
fn mixed_diff_only_hides_empty_hunks() {
    // A multi-hunk diff where the first hunk has only context and the
    // second has actual changes: only the first header should be hidden.
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,5 +1,5 @@\n a\n b\n c\n d\n e\n@@ -20,3 +20,4 @@\n x\n y\n+inserted\n w\n";
    let out = render(input, &RenderOptions::default());
    assert!(!out.contains("@@ -1,5 +1,5 @@"));
    assert!(out.contains("@@ -20,3 +20,4 @@"));
}

#[test]
fn hunk_header_hidden_for_single_line_context_only_hunk() {
    // Mirrors the user's exact case from LazyGit: a hunk whose header
    // claims 11 lines on each side but only one context line is shown.
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,11 +1,11 @@\n type ParseOptions = {\n";
    let out = render(input, &RenderOptions::default());
    assert!(
        !out.contains("@@"),
        "context-only hunk header should be hidden, got:\n{}",
        out
    );
}
