use diffview::{parse_patch, render, FileDiff, RenderOptions};

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(&nc) = chars.peek() {
                chars.next();
                if nc == 'm' {
                    break;
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

fn opts() -> RenderOptions {
    RenderOptions {
        width: 80,
        ..RenderOptions::default()
    }
}

#[test]
fn parse_simple_modification() {
    let input = "diff --git a/example.ts b/example.ts\nindex abc..def 100644\n--- a/example.ts\n+++ b/example.ts\n@@ -1,3 +1,3 @@\n line one\n-old\n+new\n line three\n";
    let files = parse_patch(input);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "example.ts");
    assert_eq!(files[0].hunks.len(), 1);
    assert_eq!(files[0].hunks[0].lines.len(), 4);
}

#[test]
fn parse_new_file() {
    let input = "diff --git a/new.txt b/new.txt\nnew file mode 100644\nindex 0000000..abc1234\n--- /dev/null\n+++ b/new.txt\n@@ -0,0 +1,2 @@\n+hello\n+world\n";
    let files = parse_patch(input);
    assert_eq!(files.len(), 1);
    assert!(matches!(files[0].state, diffview::State::New));
}

#[test]
fn parse_deleted_file() {
    let input = "diff --git a/old.txt b/old.txt\ndeleted file mode 100644\nindex abc1234..0000000\n--- a/old.txt\n+++ /dev/null\n@@ -1,2 +0,0 @@\n-bye\n-cruel world\n";
    let files = parse_patch(input);
    assert_eq!(files.len(), 1);
    assert!(matches!(files[0].state, diffview::State::Deleted));
}

#[test]
fn parse_renamed_file() {
    let input = "diff --git a/old.txt b/new.txt\nsimilarity index 100%\nrename from old.txt\nrename to new.txt\n";
    let files = parse_patch(input);
    assert_eq!(files.len(), 1);
    assert!(matches!(files[0].state, diffview::State::Renamed));
}

#[test]
fn parse_multiple_files() {
    let input = "diff --git a/a.ts b/a.ts\nindex 111..222 100644\n--- a/a.ts\n+++ b/a.ts\n@@ -1,1 +1,1 @@\n-a\n+b\ndiff --git a/b.ts b/b.ts\nindex 333..444 100644\n--- a/b.ts\n+++ b/b.ts\n@@ -1,1 +1,1 @@\n-c\n+d\n";
    let files = parse_patch(input);
    assert_eq!(files.len(), 2);
}

#[test]
fn render_emits_title_hunk_header_and_code_lines() {
    // Use a multi-hunk diff so hunk headers are preserved.
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n@@ -10,2 +10,2 @@\n const b = 3;\n-const c = 4;\n+const c = 5;\n";
    let out = render(input, &opts());
    let plain = strip_ansi(&out);
    assert!(plain.contains("x.ts"));
    assert!(out.contains("@@ -1,2 +1,2 @@"));
    assert!(plain.contains("const a = 1;"));
    assert!(plain.contains("const a = 2;"));
    assert!(!out.contains("diff --git"));
    assert!(!out.contains("--- a/"));
    assert!(!out.contains("+++ b/"));
    assert!(!out.contains("index "));
}

#[test]
fn render_title_includes_stats() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,3 @@\n keep\n-old\n+new\n end\n";
    let out = render(input, &opts());
    let lines: Vec<&str> = out.split('\n').collect();
    let title_line = strip_ansi(lines[1]);
    assert!(title_line.contains("x.ts"));
    assert!(title_line.contains("+1"));
    assert!(title_line.contains("-1"));
    assert!(out.contains("\x1b[48;2;"));
}

#[test]
fn render_new_file_title() {
    let input = "diff --git a/new.txt b/new.txt\nnew file mode 100644\nindex 0000000..abc1234\n--- /dev/null\n+++ b/new.txt\n@@ -0,0 +1,2 @@\n+hello\n+world\n";
    let out = render(input, &opts());
    let title_line = strip_ansi(out.split('\n').nth(1).unwrap());
    assert!(title_line.contains("new"));
    assert!(title_line.contains("new.txt"));
    assert!(title_line.contains("+2"));
    assert!(title_line.contains("-0"));
}

#[test]
fn render_deleted_file_title() {
    let input = "diff --git a/old.txt b/old.txt\ndeleted file mode 100644\nindex abc1234..0000000\n--- a/old.txt\n+++ /dev/null\n@@ -1,2 +0,0 @@\n-bye\n-cruel world\n";
    let out = render(input, &opts());
    let title_line = strip_ansi(out.split('\n').nth(1).unwrap());
    assert!(title_line.contains("deleted"));
    assert!(title_line.contains("old.txt"));
    assert!(title_line.contains("-2"));
}

#[test]
fn render_renamed_file_title() {
    let input = "diff --git a/old.txt b/new.txt\nsimilarity index 100%\nrename from old.txt\nrename to new.txt\n";
    let out = render(input, &opts());
    let title_line = strip_ansi(out.split('\n').nth(1).unwrap());
    assert!(title_line.contains("renamed"));
    assert!(title_line.contains("old.txt"));
    assert!(title_line.contains("new.txt"));
}

#[test]
fn render_separator_between_files_but_not_after_last() {
    let input = "diff --git a/a.ts b/a.ts\nindex 111..222 100644\n--- a/a.ts\n+++ b/a.ts\n@@ -1,1 +1,1 @@\n-a\n+b\ndiff --git a/b.ts b/b.ts\nindex 333..444 100644\n--- a/b.ts\n+++ b/b.ts\n@@ -1,1 +1,1 @@\n-c\n+d\n";
    let out = render(input, &opts());
    let plain = strip_ansi(&out);
    let separator_count = plain.matches("─").filter(|_| true).count();
    assert!(separator_count >= 30);
    // The separator character must appear between the two file sections.
    let sections: Vec<&str> = plain.split("─").collect();
    assert!(sections.len() >= 2);
    // The last line of the output is a code line, not a separator.
    let last_meaningful = plain
        .trim_end()
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap();
    assert!(!last_meaningful.starts_with('─'));
}

#[test]
fn render_gutter_on_every_code_line() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,3 @@\n keep\n-old\n+new\n end\n";
    let out = render(input, &opts());
    let gutter_bgs = [
        format!("48;2;{};{};{}",
            (diffview::DARK.meta_bg >> 16) & 0xff,
            (diffview::DARK.meta_bg >> 8) & 0xff,
            diffview::DARK.meta_bg & 0xff),
        format!("48;2;{};{};{}",
            (diffview::DARK.del_gutter_bg >> 16) & 0xff,
            (diffview::DARK.del_gutter_bg >> 8) & 0xff,
            diffview::DARK.del_gutter_bg & 0xff),
        format!("48;2;{};{};{}",
            (diffview::DARK.add_gutter_bg >> 16) & 0xff,
            (diffview::DARK.add_gutter_bg >> 8) & 0xff,
            diffview::DARK.add_gutter_bg & 0xff),
    ];
    let code_lines: Vec<&str> = out.split('\n').filter(|l| l.contains("keep") || l.contains("old") || l.contains("new") || l.contains("end")).collect();
    assert!(!code_lines.is_empty());
    for line in &code_lines {
        let has_gutter = gutter_bgs.iter().any(|bg| line.contains(bg));
        assert!(has_gutter, "expected gutter background in {}", line);
    }
}

#[test]
fn addition_deletion_fill_terminal_width() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let mut o = opts();
    o.width = 60;
    let out = render(input, &o);
    let plain = strip_ansi(&out);
    for line in plain.split('\n') {
        if line.contains("+ new") || line.contains("- old") {
            assert_eq!(line.chars().count(), 60, "code line should fill width: {:?}", line);
        }
    }
}

#[test]
fn addition_deletion_fill_width_without_line_numbers() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let mut o = opts();
    o.width = 60;
    o.no_line_numbers = true;
    let out = render(input, &o);
    let plain = strip_ansi(&out);
    for line in plain.split('\n') {
        if line.contains("+ new") || line.contains("- old") {
            assert_eq!(line.chars().count(), 60, "code line should fill width: {:?}", line);
        }
    }
}

#[test]
fn render_line_numbers_colored_for_changes() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let out = render(input, &opts());
    let plain = strip_ansi(&out);
    let addition_line = plain.split('\n').find(|l| l.contains("+ new"));
    let deletion_line = plain.split('\n').find(|l| l.contains("- old"));
    assert!(addition_line.is_some());
    assert!(deletion_line.is_some());
    let add_fg = format!(
        "38;2;{};{};{}",
        (diffview::DARK.add_accent >> 16) & 0xff,
        (diffview::DARK.add_accent >> 8) & 0xff,
        diffview::DARK.add_accent & 0xff
    );
    let del_fg = format!(
        "38;2;{};{};{}",
        (diffview::DARK.del_accent >> 16) & 0xff,
        (diffview::DARK.del_accent >> 8) & 0xff,
        diffview::DARK.del_accent & 0xff
    );
    let raw_add = out.split('\n').find(|l| l.contains("new") && l.contains("+")).unwrap();
    let raw_del = out.split('\n').find(|l| l.contains("old") && l.contains("-")).unwrap();
    assert!(raw_add.contains(&add_fg), "addition line number should use add_accent fg: {}", raw_add);
    assert!(raw_del.contains(&del_fg), "deletion line number should use del_accent fg: {}", raw_del);
}

#[test]
fn render_title_is_three_lines() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let mut o = opts();
    o.width = 40;
    let out = render(input, &o);
    let lines: Vec<&str> = out.split('\n').collect();
    let title_bg = "48;2;43;49;56";
    assert!(lines[0].contains(title_bg));
    assert!(lines[0].contains(" ".repeat(40).as_str()) || strip_ansi(lines[0]).len() == 40);
    assert!(lines[2].contains(title_bg));
    let title_plain = strip_ansi(lines[1]);
    assert_eq!(title_plain.chars().count(), 40);
    assert!(title_plain.contains("x.ts"));
}

#[test]
fn render_title_fills_configured_width() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let mut o = opts();
    o.width = 50;
    let out = render(input, &o);
    let lines: Vec<&str> = out.split('\n').collect();
    assert_eq!(strip_ansi(lines[1]).chars().count(), 50);
    assert_eq!(strip_ansi(lines[0]).chars().count(), 50);
    assert_eq!(strip_ansi(lines[2]).chars().count(), 50);
}

#[test]
fn render_title_has_nerd_font_icon() {
    // .ts files get the TypeScript codicon (U+E628) before the filename.
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old\n+new\n";
    let mut o = opts();
    o.width = 50;
    let out = render(input, &o);
    let title_plain = strip_ansi(out.split('\n').nth(1).unwrap());
    assert!(title_plain.contains('\u{e628}'));
    let icon_idx = title_plain.find('\u{e628}').unwrap();
    let x_idx = title_plain.find("x.ts").unwrap();
    assert!(icon_idx < x_idx);
}

#[test]
fn render_title_uses_per_file_icon() {
    // Different extensions pick different nerd-font codepoints.
    let o = || {
        let mut r = opts();
        r.width = 50;
        r
    };
    let ts = render(
        "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-a\n+b\n",
        &o(),
    );
    let rs = render(
        "diff --git a/x.rs b/x.rs\n--- a/x.rs\n+++ b/x.rs\n@@ -1,1 +1,1 @@\n-a\n+b\n",
        &o(),
    );
    let py = render(
        "diff --git a/x.py b/x.py\n--- a/x.py\n+++ b/x.py\n@@ -1,1 +1,1 @@\n-a\n+b\n",
        &o(),
    );
    let ts_plain = strip_ansi(ts.split('\n').nth(1).unwrap());
    let rs_plain = strip_ansi(rs.split('\n').nth(1).unwrap());
    let py_plain = strip_ansi(py.split('\n').nth(1).unwrap());
    assert!(ts_plain.contains('\u{e628}')); // TS (nvim-web-devicons v3)
    assert!(rs_plain.contains('\u{e68b}')); // Rust (nvim-web-devicons v3)
    assert!(py_plain.contains('\u{e606}')); // Python (nvim-web-devicons v3)
    assert_ne!(ts_plain, rs_plain);
}

#[test]
fn render_line_numbers_advance_after_change() {
    let lines = vec![
        "diff --git a/about.mdx b/about.mdx",
        "index 1234567..abcdef0 100644",
        "--- a/about.mdx",
        "+++ b/about.mdx",
        "@@ -15,6 +15,12 @@ Hi there",
        " line A",
        " line B",
        " line C",
        "+asldkjj mejor que nunca jajaja pero es es o jaja",
        "+",
        "+",
        "+",
        "+",
        "+",
        " line D",
        " line E",
        " line F",
    ];
    let text = lines.join("\n") + "\n";
    let mut o = opts();
    o.width = 50;
    let out = render(&text, &o);
    let plain = strip_ansi(&out);
    let plain_lines: Vec<&str> = plain.split('\n').collect();
    let line_d = plain_lines.iter().find(|l| l.contains(" line D")).unwrap();
    let line_f = plain_lines.iter().find(|l| l.contains(" line F")).unwrap();
    let d_match: Vec<&str> = line_d.matches(|c: char| c.is_ascii_digit()).collect();
    let f_match: Vec<&str> = line_f.matches(|c: char| c.is_ascii_digit()).collect();
    let d_old: i32 = d_match[..2].join("").parse().unwrap();
    let d_new: i32 = d_match[2..4].join("").parse().unwrap();
    let f_old: i32 = f_match[..2].join("").parse().unwrap();
    let f_new: i32 = f_match[2..4].join("").parse().unwrap();
    assert_eq!(d_old, 18);
    assert_eq!(d_new, 24);
    assert_eq!(f_old, 20);
    assert_eq!(f_new, 26);
}

#[test]
fn render_line_numbers_advance_across_multiple_changes() {
    let lines = vec![
        "diff --git a/x.ts b/x.ts",
        "index 111..222 100644",
        "--- a/x.ts",
        "+++ b/x.ts",
        "@@ -1,5 +1,5 @@ header",
        " keep A",
        "-del1",
        "+add1",
        " keep B",
        "-del2",
        "+add2",
        " keep C",
    ];
    let text = lines.join("\n") + "\n";
    let mut o = opts();
    o.width = 50;
    let out = render(&text, &o);
    let raw_lines: Vec<&str> = out.split('\n').collect();

    fn gutter(raw_lines: &[&str], marker: &str) -> (Option<i32>, Option<i32>) {
        let line = raw_lines
            .iter()
            .find(|l| {
                let plain = strip_ansi(l);
                plain.contains(marker)
            })
            .unwrap();
        let plain = strip_ansi(line);
        let old_s: String = plain.chars().take(4).collect();
        let new_s: String = plain.chars().skip(5).take(4).collect();
        let old = old_s.trim().parse::<i32>().ok();
        let new = new_s.trim().parse::<i32>().ok();
        (old, new)
    }

    assert_eq!(gutter(&raw_lines, "  keep A"), (Some(1), Some(1)));
    assert_eq!(gutter(&raw_lines, "- del1"), (Some(2), None));
    assert_eq!(gutter(&raw_lines, "+ add1"), (None, Some(2)));
    assert_eq!(gutter(&raw_lines, "  keep B"), (Some(3), Some(3)));
    assert_eq!(gutter(&raw_lines, "- del2"), (Some(4), None));
    assert_eq!(gutter(&raw_lines, "+ add2"), (None, Some(4)));
    assert_eq!(gutter(&raw_lines, "  keep C"), (Some(5), Some(5)));
}

#[test]
fn render_hides_line_numbers_when_requested() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,3 @@\n keep\n-old\n+new\n end\n";
    let mut o = opts();
    o.no_line_numbers = true;
    let out = render(input, &o);
    let plain = strip_ansi(&out);
    for line in plain.split('\n') {
        assert!(!line.contains('│'), "unexpected gutter in {}", line);
    }
}

#[test]
fn render_addition_line_numbers_painted_green() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,3 +1,3 @@\n keep\n-old\n+new\n end\n";
    let out = render(input, &opts());
    let add_fg = format!(
        "38;2;{};{};{}",
        (diffview::DARK.add_accent >> 16) & 0xff,
        (diffview::DARK.add_accent >> 8) & 0xff,
        diffview::DARK.add_accent & 0xff
    );
    let del_fg = format!(
        "38;2;{};{};{}",
        (diffview::DARK.del_accent >> 16) & 0xff,
        (diffview::DARK.del_accent >> 8) & 0xff,
        diffview::DARK.del_accent & 0xff
    );
    let raw_add = out.split('\n').find(|l| l.contains("new") && l.contains("+")).unwrap();
    let raw_del = out.split('\n').find(|l| l.contains("old") && l.contains("-")).unwrap();
    assert!(raw_add.contains(&add_fg), "addition line number should use add_accent fg: {}", raw_add);
    assert!(raw_del.contains(&del_fg), "deletion line number should use del_accent fg: {}", raw_del);
}

#[test]
fn render_separator_present_for_multi_file() {
    let input = "diff --git a/a.ts b/a.ts\nindex 111..222 100644\n--- a/a.ts\n+++ b/a.ts\n@@ -1,1 +1,1 @@\n-a\n+b\ndiff --git a/b.ts b/b.ts\nindex 333..444 100644\n--- a/b.ts\n+++ b/b.ts\n@@ -1,1 +1,1 @@\n-c\n+d\n";
    let out = render(input, &opts());
    assert!(out.contains("─"));
}

#[test]
fn char_count_handles_utf8() {
    use diffview::char_count;
    let s = format!("{} example.ts", diffview::FILE_ICON);
    assert_eq!(char_count(&s), 1 + 1 + 10);
}

#[test]
fn char_count_skips_ansi() {
    use diffview::char_count;
    let s = "\x1b[0m\x1b[38;2;1;2;3mhello\x1b[0m";
    assert_eq!(char_count(s), 5);
}

#[test]
fn hunk_header_followed_by_code_line() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n@@ -10,2 +10,2 @@\n const b = 3;\n-const c = 4;\n+const c = 5;\n";
    let out = render(input, &opts());
    let plain = strip_ansi(&out);
    let lines: Vec<&str> = plain.lines().collect();
    let mut header_count = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("@@") {
            header_count += 1;
            assert!(i + 1 < lines.len(), "hunk header has no following line");
            let next = &lines[i + 1];
            assert!(
                !next.is_empty() && !next.contains("@@"),
                "blank or hunk line immediately after hunk header: got {:?}",
                next
            );
        }
    }
    assert_eq!(header_count, 2);
}

#[test]
fn hunk_header_indented_to_source_text_column() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n@@ -10,2 +10,2 @@\n const b = 3;\n-const c = 4;\n+const c = 5;\n";
    let out = render(input, &opts());
    let plain = strip_ansi(&out);
    let hunk_line = plain.lines().find(|line| line.contains("@@")).unwrap();
    assert_eq!(hunk_line.chars().position(|c| c == '@'), Some(12));
}

#[test]
fn hunk_header_indent_without_line_numbers() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n@@ -10,2 +10,2 @@\n const b = 3;\n-const c = 4;\n+const c = 5;\n";
    let mut o = opts();
    o.no_line_numbers = true;
    let out = render(input, &o);
    let plain = strip_ansi(&out);
    let hunk_line = plain.lines().find(|line| line.contains("@@")).unwrap();
    let leading_spaces = hunk_line.len() - hunk_line.trim_start().len();
    assert_eq!(leading_spaces, 2);
}

#[test]
fn hunk_header_row_fills_width() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n@@ -10,2 +10,2 @@\n const b = 3;\n-const c = 4;\n+const c = 5;\n";
    let mut o = opts();
    o.width = 60;
    let out = render(input, &o);
    let plain = strip_ansi(&out);
    let hunk_line = plain.lines().find(|line| line.contains("@@")).unwrap();
    assert_eq!(hunk_line.chars().count(), 60);
}

#[test]
fn hunk_header_full_row_has_background() {
    let input = "diff --git a/x.ts b/x.ts\nindex abc..def 100644\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-const a = 1;\n+const a = 2;\n";
    let mut o = opts();
    o.width = 50;
    let out = render(input, &o);
    let hunk_bg = "48;2;13;44;69";
    let hunk_gutter_bg = "48;2;22;74;112";
    let lines: Vec<&str> = out.split('\n').collect();
    let hunk_line = lines.iter().find(|l| l.contains("@@")).unwrap();
    let stripped = strip_ansi(hunk_line);
    assert_eq!(stripped.chars().count(), 50);
    let hunk_bg_count = hunk_line.matches(hunk_bg).count();
    let gutter_bg_count = hunk_line.matches(hunk_gutter_bg).count();
    assert!(
        hunk_bg_count >= 1,
        "hunk_bg should cover text + padding"
    );
    assert!(
        gutter_bg_count >= 1,
        "hunk_gutter_bg should cover the gutter prefix"
    );
}

#[test]
fn hunk_header_uses_muted_foreground() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-old\n+new\n";
    let out = render(input, &opts());
    let muted_fg = format!(
        "38;2;{};{};{}",
        (diffview::DARK.hunk_fg >> 16) & 0xff,
        (diffview::DARK.hunk_fg >> 8) & 0xff,
        diffview::DARK.hunk_fg & 0xff
    );
    let hunk_line = out.split('\n').find(|l| l.contains("@@")).unwrap();
    assert!(hunk_line.contains(&muted_fg), "hunk header should use muted hunk_fg, line={}", hunk_line);
}

#[test]
fn hunk_header_uses_start_indicator_at_start_of_file() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,2 +1,2 @@\n-old\n+new\n@@ -10,1 +10,1 @@\n-old ten\n+new ten\n";
    let plain = strip_ansi(&render(input, &opts()));
    let hunk_line = plain.lines().find(|line| line.contains("@@ -1,2")).unwrap();
    assert_eq!(
        hunk_line.chars().take(12).collect::<String>(),
        "     󰇘      "
    );
}

#[test]
fn hunk_header_uses_up_indicator_for_hidden_context_above() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -10,1 +10,1 @@\n-old\n+new\n";
    let plain = strip_ansi(&render(input, &opts()));
    let hunk_line = plain.lines().find(|line| line.contains("@@")).unwrap();
    assert_eq!(
        hunk_line.chars().take(12).collect::<String>(),
        "     ↑      "
    );
}

#[test]
fn hunk_header_uses_both_indicator_for_hidden_context_on_both_sides() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -10,1 +10,1 @@\n-old ten\n+new ten\n@@ -30,1 +30,1 @@\n-old thirty\n+new thirty\n";
    let plain = strip_ansi(&render(input, &opts()));
    let hunk_line = plain
        .lines()
        .find(|line| line.contains("@@ -10,1"))
        .unwrap();
    assert_eq!(
        hunk_line.chars().take(12).collect::<String>(),
        "     󰹹      "
    );
    assert_eq!(hunk_line.chars().position(|c| c == '@'), Some(12));
}

#[test]
fn hunk_header_uses_down_indicator_when_only_context_below_is_proven() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old one\n+new one\n@@ -2,1 +2,1 @@\n-old two\n+new two\n@@ -10,1 +10,1 @@\n-old ten\n+new ten\n";
    let plain = strip_ansi(&render(input, &opts()));
    let hunk_line = plain.lines().find(|line| line.contains("@@ -2,1")).unwrap();
    assert_eq!(
        hunk_line.chars().take(12).collect::<String>(),
        "     ↓      "
    );
}

#[test]
fn hunk_header_omits_indicator_when_direction_is_unknown() {
    let input = "diff --git a/x.ts b/x.ts\n--- a/x.ts\n+++ b/x.ts\n@@ -1,1 +1,1 @@\n-old one\n+new one\n@@ -2,1 +2,1 @@\n-old two\n+new two\n";
    let plain = strip_ansi(&render(input, &opts()));
    let hunk_line = plain.lines().find(|line| line.contains("@@ -2,1")).unwrap();
    assert_eq!(
        hunk_line.chars().take(12).collect::<String>(),
        "            "
    );
}
