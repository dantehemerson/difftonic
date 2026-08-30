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
