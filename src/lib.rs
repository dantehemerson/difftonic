use std::path::Path;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

mod highlight;
mod icons;

pub const RESET: &str = "\x1b[0m";
pub const FILE_ICON: &str = "\u{f15b}";
pub const DEFAULT_WIDTH: usize = 80;

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub syntax_theme: String,
    pub theme: String,
    pub no_line_numbers: bool,
    pub full: bool,
    pub width: usize,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            syntax_theme: "github-dark-default".into(),
            theme: "auto".into(),
            no_line_numbers: false,
            full: false,
            width: DEFAULT_WIDTH,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub meta_bg: u32,
    pub meta_fg: u32,
    pub hunk_bg: u32,
    pub hunk_fg: u32,
    pub header_bg: u32,
    pub header_fg: u32,
    pub header_muted: u32,
    pub separator: u32,
    pub rail: u32,
    pub add_bg: u32,
    pub del_bg: u32,
    pub add_accent: u32,
    pub del_accent: u32,
    pub syntax: Syntax,
}

#[derive(Clone, Copy)]
pub struct Syntax {
    pub comment: u32,
    pub keyword: u32,
    pub string: u32,
    pub string_special: u32,
    pub number: u32,
    pub constant_builtin: u32,
    pub function: u32,
    pub function_method: u32,
    pub function_macro: u32,
    pub type_: u32,
    pub type_builtin: u32,
    pub constructor: u32,
    pub variable: u32,
    pub variable_builtin: u32,
    pub variable_parameter: u32,
    pub variable_member: u32,
    pub property: u32,
    pub module: u32,
    pub operator: u32,
    pub tag: u32,
    pub attribute: u32,
    pub label: u32,
    pub punctuation: u32,
    pub default: u32,
}

pub const DARK: Theme = Theme {
    meta_bg: 0x1f2228,
    meta_fg: 0x9da0a6,
    hunk_bg: 0x0d2c45,
    hunk_fg: 0xdceefb,
    header_bg: 0x2b3138,
    header_fg: 0xe6edf3,
    header_muted: 0x8b949e,
    separator: 0x4a4a4a,
    rail: 0x4a4a4a,
    add_bg: 0x0e3017,
    del_bg: 0x350a0d,
    add_accent: 0x86d687,
    del_accent: 0xed9b9b,
    syntax: Syntax {
        comment: 0x7f848e,
        keyword: 0xc678dd,
        string: 0x98c379,
        string_special: 0x56b6c2,
        number: 0xd19a66,
        constant_builtin: 0xd19a66,
        function: 0x61afef,
        function_method: 0x61afef,
        function_macro: 0xc678dd,
        type_: 0xe5c07b,
        type_builtin: 0xe5c07b,
        constructor: 0xe5c07b,
        variable: 0xe06c75,
        variable_builtin: 0xe06c75,
        variable_parameter: 0xe06c75,
        variable_member: 0xe06c75,
        property: 0xe06c75,
        module: 0xe5c07b,
        operator: 0x56b6c2,
        tag: 0xe06c75,
        attribute: 0xd19a66,
        label: 0x56b6c2,
        punctuation: 0xabb2bf,
        default: 0xabb2bf,
    },
};

pub const LIGHT: Theme = Theme {
    meta_bg: 0xe6e6e6,
    meta_fg: 0x555555,
    hunk_bg: 0xb6dcf5,
    hunk_fg: 0x073a5e,
    header_bg: 0xd9e1e8,
    header_fg: 0x0d1117,
    header_muted: 0x57606a,
    separator: 0xb0b0b0,
    rail: 0xb0b0b0,
    add_bg: 0xdbefdc,
    del_bg: 0xf3d8d8,
    add_accent: 0x2c7a2c,
    del_accent: 0xa93232,
    syntax: Syntax {
        comment: 0x8b949e,
        keyword: 0xa626a4,
        string: 0x50a14f,
        string_special: 0x0184bc,
        number: 0x986801,
        constant_builtin: 0x986801,
        function: 0x4078f2,
        function_method: 0x4078f2,
        function_macro: 0xa626a4,
        type_: 0xc18401,
        type_builtin: 0xc18401,
        constructor: 0xc18401,
        variable: 0xe45649,
        variable_builtin: 0xe45649,
        variable_parameter: 0xe45649,
        variable_member: 0xe45649,
        property: 0xe45649,
        module: 0xc18401,
        operator: 0x0184bc,
        tag: 0xe45649,
        attribute: 0x986801,
        label: 0x0184bc,
        punctuation: 0x383a42,
        default: 0x383a42,
    },
};

#[derive(Debug)]
pub struct FileDiff {
    pub name: String,
    pub previous: Option<String>,
    pub state: State,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    New,
    Deleted,
    Renamed,
}

#[derive(Debug)]
pub struct Hunk {
    pub header: String,
    pub old_start: usize,
    pub new_start: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: Kind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Context,
    Addition,
    Deletion,
    NoNewline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub color: u32,
}

pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "label",
    "module",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.member",
];

pub struct SyntaxHighlighter {
    config: Option<HighlightConfiguration>,
}

impl SyntaxHighlighter {
    pub fn new(path: &str) -> Self {
        Self {
            config: make_config(path),
        }
    }

    pub fn highlight(&mut self, source: &str, syntax: Syntax) -> Option<Vec<Vec<Token>>> {
        let config = self.config.as_mut()?;
        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(config, source.as_bytes(), None, |_| None)
            .ok()?;
        let mut lines: Vec<Vec<Token>> = vec![Vec::new()];
        let mut active: Option<u32> = None;
        for event in events.flatten() {
            match event {
                HighlightEvent::HighlightStart(h) => active = Some(h.0 as u32),
                HighlightEvent::HighlightEnd => active = None,
                HighlightEvent::Source { start, end } => {
                    let part = &source[start..end];
                    let pieces: Vec<&str> = part.split('\n').collect();
                    for (i, piece) in pieces.iter().enumerate() {
                        if !piece.is_empty() {
                            let color = active
                                .map(|idx| highlight_color(idx, syntax))
                                .unwrap_or(syntax.default);
                            lines.last_mut().unwrap().push(Token {
                                text: piece.to_string(),
                                color,
                            });
                        }
                        if i + 1 < pieces.len() {
                            lines.push(Vec::new());
                        }
                    }
                }
            }
        }
        Some(lines)
    }
}

fn highlight_color(index: u32, syntax: Syntax) -> u32 {
    match HIGHLIGHT_NAMES.get(index as usize).copied() {
        Some("comment") => syntax.comment,
        Some("keyword") => syntax.keyword,
        Some("string") => syntax.string,
        Some("string.special") => syntax.string_special,
        Some("number") | Some("constant") | Some("constant.builtin") => syntax.number,
        Some("function") | Some("function.builtin") => syntax.function,
        Some("function.method") => syntax.function_method,
        Some("function.macro") => syntax.function_macro,
        Some("type") => syntax.type_,
        Some("type.builtin") => syntax.type_builtin,
        Some("constructor") => syntax.constructor,
        Some("variable") => syntax.variable,
        Some("variable.builtin") => syntax.variable_builtin,
        Some("variable.parameter") => syntax.variable_parameter,
        Some("variable.member") => syntax.variable_member,
        Some("property") => syntax.property,
        Some("module") => syntax.module,
        Some("operator") => syntax.operator,
        Some("tag") => syntax.tag,
        Some("attribute") => syntax.attribute,
        Some("label") => syntax.label,
        Some("punctuation") | Some("punctuation.bracket") | Some("punctuation.delimiter") => {
            syntax.punctuation
        }
        _ => syntax.default,
    }
}

pub fn make_config(path: &str) -> Option<HighlightConfiguration> {
    let basename = Path::new(path).file_name()?.to_str()?;
    let overrides: &[(&str, &str)] = &[("Dockerfile", "dockerfile"), ("Makefile", "makefile")];
    for (name, ext) in overrides {
        if basename == *name {
            return make_for_ext(ext);
        }
    }
    let ext = Path::new(path).extension()?.to_str()?;
    make_for_ext(ext)
}

pub fn make_for_ext(ext: &str) -> Option<HighlightConfiguration> {
    use highlight::queries as q;

    let (language, query): (tree_sitter::Language, &str) = match ext {
        "ts" | "mts" | "cts" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            q::TS_HIGHLIGHTS,
        ),
        "tsx" => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            q::TSX_HIGHLIGHTS,
        ),
        "js" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), q::JS_HIGHLIGHTS),
        "rs" => (tree_sitter_rust::LANGUAGE.into(), q::RUST_HIGHLIGHTS),
        "py" => (tree_sitter_python::LANGUAGE.into(), q::PYTHON_HIGHLIGHTS),
        "go" => (tree_sitter_go::LANGUAGE.into(), q::GO_HIGHLIGHTS),
        "json" => (tree_sitter_json::LANGUAGE.into(), q::JSON_HIGHLIGHTS),
        "css" => (
            tree_sitter_css::LANGUAGE.into(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
        ),
        "html" => (
            tree_sitter_html::LANGUAGE.into(),
            tree_sitter_html::HIGHLIGHTS_QUERY,
        ),
        "sh" | "bash" => (tree_sitter_bash::LANGUAGE.into(), q::BASH_HIGHLIGHTS),
        "md" | "mdx" => return None,
        "dockerfile" => return None,
        "makefile" => return None,
        _ => return None,
    };
    let mut config = HighlightConfiguration::new(language, ext, query, "", "").ok()?;
    config.configure(HIGHLIGHT_NAMES);
    Some(config)
}

pub fn is_light_theme(name: &str) -> bool {
    let light_names = [
        "github-light",
        "github-light-default",
        "github-light-high-contrast",
        "light-plus",
        "solarized-light",
        "min-light",
        "one-light",
        "rose-pine-dawn",
        "slack-ochin",
        "snazzy-light",
        "vitesse-light",
        "material-theme-lighter",
        "catppuccin-latte",
        "nord-light",
        "ayu-light",
        "kleur-light",
    ];
    light_names.contains(&name)
}

pub fn paint(text: &str, bg: Option<u32>, fg: Option<u32>, bold: bool, dim: bool) -> String {
    let mut codes: Vec<String> = Vec::new();
    if let Some(c) = bg {
        codes.push(format!(
            "48;2;{};{};{}",
            (c >> 16) & 255,
            (c >> 8) & 255,
            c & 255
        ));
    }
    if let Some(c) = fg {
        codes.push(format!(
            "38;2;{};{};{}",
            (c >> 16) & 255,
            (c >> 8) & 255,
            c & 255
        ));
    }
    if bold {
        codes.push("1".into());
    }
    if dim {
        codes.push("2".into());
    }
    if codes.is_empty() {
        text.to_string()
    } else {
        format!("{}\x1b[{}m{}{}", RESET, codes.join(";"), text, RESET)
    }
}

pub fn pad(n: usize, w: usize) -> String {
    let s = n.to_string();
    if s.len() >= w {
        s
    } else {
        format!("{}{}", " ".repeat(w - s.len()), s)
    }
}

pub fn parse_patch(input: &str) -> Vec<FileDiff> {
    let normalized = input.replace('\r', "");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let mut files = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if !lines[i].starts_with("diff --git ") {
            i += 1;
            continue;
        }
        let header = lines[i];
        let parts: Vec<&str> = header.split_whitespace().collect();
        let old = parts
            .get(2)
            .unwrap_or(&"a/unknown")
            .trim_start_matches("a/")
            .to_string();
        let new = parts
            .get(3)
            .unwrap_or(&"b/unknown")
            .trim_start_matches("b/")
            .to_string();
        let mut file = FileDiff {
            name: new.clone(),
            previous: None,
            state: State::Normal,
            hunks: Vec::new(),
        };
        i += 1;
        while i < lines.len() && !lines[i].starts_with("diff --git ") {
            let line = lines[i];
            if line.starts_with("new file mode") {
                file.state = State::New;
            } else if line.starts_with("deleted file mode") {
                file.state = State::Deleted;
                file.name = old.clone();
            } else if let Some(value) = line.strip_prefix("rename from ") {
                file.previous = Some(value.to_string());
                file.state = State::Renamed;
            } else if let Some(value) = line.strip_prefix("rename to ") {
                file.name = value.to_string();
                file.state = State::Renamed;
            } else if line.starts_with("@@ ") {
                let (hunk, next) = parse_hunk(&lines, i);
                file.hunks.push(hunk);
                i = next;
                continue;
            }
            i += 1;
        }
        files.push(file);
    }
    files
}

pub fn parse_hunk(lines: &[&str], start: usize) -> (Hunk, usize) {
    let header = lines[start].to_string();
    let (old_start, new_start) = parse_hunk_numbers(&header);
    let mut body = Vec::new();
    let mut i = start + 1;
    while i < lines.len() && !lines[i].starts_with("@@ ") && !lines[i].starts_with("diff --git ") {
        let raw = lines[i];
        if raw.starts_with('\\') {
            body.push(DiffLine {
                kind: Kind::NoNewline,
                text: raw.to_string(),
            });
        } else if let Some(text) = raw.strip_prefix('+') {
            body.push(DiffLine {
                kind: Kind::Addition,
                text: text.to_string(),
            });
        } else if let Some(text) = raw.strip_prefix('-') {
            body.push(DiffLine {
                kind: Kind::Deletion,
                text: text.to_string(),
            });
        } else if let Some(text) = raw.strip_prefix(' ') {
            body.push(DiffLine {
                kind: Kind::Context,
                text: text.to_string(),
            });
        }
        i += 1;
    }
    (
        Hunk {
            header,
            old_start,
            new_start,
            lines: body,
        },
        i,
    )
}

pub fn parse_hunk_numbers(header: &str) -> (usize, usize) {
    let parts: Vec<&str> = header.split_whitespace().collect();
    let old = parts
        .get(1)
        .unwrap_or(&"-1")
        .trim_start_matches('-')
        .split(',')
        .next()
        .unwrap_or("1")
        .parse()
        .unwrap_or(1);
    let new = parts
        .get(2)
        .unwrap_or(&"+1")
        .trim_start_matches('+')
        .split(',')
        .next()
        .unwrap_or("1")
        .parse()
        .unwrap_or(1);
    (old, new)
}

pub fn resolve_theme(options: &RenderOptions) -> Theme {
    let light = options.theme == "light"
        || (options.theme == "auto" && is_light_theme(&options.syntax_theme));
    if light {
        LIGHT
    } else {
        DARK
    }
}

pub fn render(input: &str, options: &RenderOptions) -> String {
    let files = parse_patch(input);
    let theme = resolve_theme(options);
    let mut output = String::new();
    for (index, file) in files.iter().enumerate() {
        if index > 0 {
            output.push_str(&paint(
                &"─".repeat(options.width),
                None,
                Some(theme.separator),
                false,
                true,
            ));
            output.push('\n');
            output.push('\n');
        }
        render_file(file, &mut output, theme, options);
    }
    output
}

pub fn render_file(file: &FileDiff, out: &mut String, theme: Theme, options: &RenderOptions) {
    let width = options.width;
    let additions = file
        .hunks
        .iter()
        .flat_map(|h| &h.lines)
        .filter(|l| l.kind == Kind::Addition)
        .count();
    let deletions = file
        .hunks
        .iter()
        .flat_map(|h| &h.lines)
        .filter(|l| l.kind == Kind::Deletion)
        .count();
    let display = file
        .previous
        .as_ref()
        .filter(|p| *p != &file.name)
        .map(|p| format!("{} → {}", p, file.name))
        .unwrap_or_else(|| file.name.clone());
    let label = match file.state {
        State::New => Some("new"),
        State::Deleted => Some("deleted"),
        State::Renamed => Some("renamed"),
        State::Normal => None,
    };

    let bg_line = paint(
        &" ".repeat(width),
        Some(theme.header_bg),
        None,
        false,
        false,
    );

    let prefix_len = char_count(&format!("{} {}", icons::file_icon(&file.name), display));

    let mut stats_parts: Vec<(String, u32, bool)> = Vec::new();
    if let Some(l) = label {
        stats_parts.push((l.to_string(), theme.header_muted, false));
    }
    stats_parts.push((format!("+{}", additions), theme.add_accent, false));
    stats_parts.push((format!("-{}", deletions), theme.del_accent, false));
    let stats_rendered = render_stats(&stats_parts, theme);
    let stats_len = char_count(&stats_rendered);

    let side_pad = 1;
    let middle = width
        .saturating_sub(prefix_len + stats_len + side_pad * 2)
        .max(1);

    let mut title = String::new();
    title.push_str(&paint(
        &" ".repeat(side_pad),
        Some(theme.header_bg),
        None,
        false,
        false,
    ));
    // Icon is painted with its per-file color (Seti UI palette) while
    // the filename keeps the theme's header foreground.
    title.push_str(&paint(
        icons::file_icon(&file.name),
        Some(theme.header_bg),
        Some(icons::file_color(&file.name)),
        false,
        false,
    ));
    title.push_str(&paint(
        &format!(" {}", display),
        Some(theme.header_bg),
        Some(theme.header_fg),
        true,
        false,
    ));
    title.push_str(&paint(
        &" ".repeat(middle),
        Some(theme.header_bg),
        None,
        false,
        false,
    ));
    title.push_str(&stats_rendered);
    title.push_str(&paint(
        &" ".repeat(side_pad),
        Some(theme.header_bg),
        None,
        false,
        false,
    ));
    let current_len = char_count(&title);
    if current_len < width {
        title.push_str(&paint(
            &" ".repeat(width - current_len),
            Some(theme.header_bg),
            None,
            false,
            false,
        ));
    }

    out.push_str(&bg_line);
    out.push('\n');
    out.push_str(&title);
    out.push('\n');
    out.push_str(&bg_line);
    out.push('\n');

    let mut highlighter = SyntaxHighlighter::new(&file.name);
    let highlight_code: String = file
        .hunks
        .iter()
        .flat_map(|h| &h.lines)
        .filter(|l| l.kind != Kind::NoNewline && l.kind != Kind::Context)
        .map(|l| l.text.as_str())
        .collect::<Vec<&str>>()
        .join("\n");
    let mut token_lines: std::collections::VecDeque<Vec<Token>> = std::collections::VecDeque::new();
    if let Some(lines) = highlighter.highlight(&highlight_code, theme.syntax) {
        for l in lines {
            token_lines.push_back(l);
        }
    }

    for (hunk_index, hunk) in file.hunks.iter().enumerate() {
        if !should_hide_hunk_header(file, hunk) {
            let indicator = if options.no_line_numbers {
                ""
            } else {
                hunk_indicator(&file.hunks, hunk_index)
            };
            let hunk_prefix = hunk_prefix(indicator, !options.no_line_numbers);
            let hunk_indent = char_count(&hunk_prefix);
            let hunk_text_len = char_count(&hunk.header);
            let padding_after = width.saturating_sub(hunk_indent + hunk_text_len);
            out.push_str(&paint(
                &hunk_prefix,
                Some(theme.hunk_bg),
                Some(theme.hunk_fg),
                false,
                false,
            ));
            out.push_str(&paint(
                &hunk.header,
                Some(theme.hunk_bg),
                Some(theme.hunk_fg),
                false,
                false,
            ));
            out.push_str(&paint(
                &" ".repeat(padding_after),
                Some(theme.hunk_bg),
                None,
                false,
                false,
            ));
            out.push('\n');
        }
        let mut old = hunk.old_start;
        let mut new = hunk.new_start;
        for line in &hunk.lines {
            if line.kind == Kind::NoNewline {
                out.push_str(&paint(
                    &format!("{}\n", line.text),
                    None,
                    Some(theme.meta_fg),
                    false,
                    true,
                ));
                out.push('\n');
                continue;
            }
            let old_num = if line.kind != Kind::Addition {
                let n = old;
                old += 1;
                Some(n)
            } else {
                None
            };
            let new_num = if line.kind != Kind::Deletion {
                let n = new;
                new += 1;
                Some(n)
            } else {
                None
            };
            let should_highlight = options.full || line.kind != Kind::Context;
            let tokens: Vec<Token> = if should_highlight {
                token_lines.pop_front().unwrap_or_default()
            } else {
                Vec::new()
            };
            out.push_str(&render_line(
                line,
                old_num,
                new_num,
                &tokens,
                theme,
                !options.no_line_numbers,
            ));
            out.push('\n');
        }
    }
}

const HUNK_INDENT: usize = 13;
const HUNK_START_INDICATOR: &str = "󰇘";
const HUNK_BOTH_INDICATOR: &str = "󰹹";

fn hunk_prefix(indicator: &str, line_numbers: bool) -> String {
    if !line_numbers {
        return "  ".to_string();
    }

    let indicator_width = char_count(indicator);
    let padding = HUNK_INDENT.saturating_sub(indicator_width);
    let padding_before = padding / 2;
    let padding_after = padding - padding_before;
    format!(
        "{}{}{}",
        " ".repeat(padding_before),
        indicator,
        " ".repeat(padding_after)
    )
}

fn hunk_indicator(hunks: &[Hunk], index: usize) -> &'static str {
    let hunk = &hunks[index];
    if hunk.old_start <= 1 && hunk.new_start <= 1 {
        return HUNK_START_INDICATOR;
    }

    let hidden_above = if index == 0 {
        hunk.old_start > 1 || hunk.new_start > 1
    } else {
        has_hidden_gap(&hunks[index - 1], hunk)
    };
    let hidden_below = hunks
        .get(index + 1)
        .is_some_and(|next| has_hidden_gap(hunk, next));

    match (hidden_above, hidden_below) {
        (true, true) => HUNK_BOTH_INDICATOR,
        (true, false) => "↑",
        (false, true) => "↓",
        (false, false) => "",
    }
}

fn has_hidden_gap(previous: &Hunk, next: &Hunk) -> bool {
    let (old_count, new_count) = hunk_line_counts(previous);
    next.old_start > previous.old_start + old_count
        || next.new_start > previous.new_start + new_count
}

fn hunk_line_counts(hunk: &Hunk) -> (usize, usize) {
    let old_count = hunk
        .lines
        .iter()
        .filter(|line| matches!(line.kind, Kind::Context | Kind::Deletion))
        .count();
    let new_count = hunk
        .lines
        .iter()
        .filter(|line| matches!(line.kind, Kind::Context | Kind::Addition))
        .count();
    (old_count, new_count)
}

pub fn render_stats(parts: &[(String, u32, bool)], theme: Theme) -> String {
    let mut out = String::new();
    for (i, (text, color, bold)) in parts.iter().enumerate() {
        if i > 0 {
            out.push_str(&paint(
                " ",
                Some(theme.header_bg),
                Some(theme.header_muted),
                false,
                true,
            ));
        }
        out.push_str(&paint(
            text,
            Some(theme.header_bg),
            Some(*color),
            *bold,
            false,
        ));
    }
    out
}

pub fn char_count(s: &str) -> usize {
    let mut n = 0;
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
        n += 1;
    }
    n
}

pub fn render_line(
    line: &DiffLine,
    old: Option<usize>,
    new: Option<usize>,
    tokens: &[Token],
    t: Theme,
    numbers: bool,
) -> String {
    let mut out = String::new();
    let rail_color = match line.kind {
        Kind::Addition => t.add_accent,
        Kind::Deletion => t.del_accent,
        _ => t.rail,
    };
    let rail_bold = line.kind != Kind::Context;
    out.push_str(&paint("▌", None, Some(rail_color), rail_bold, !rail_bold));

    if numbers {
        let old_s = old.map(|n| pad(n, 4)).unwrap_or_else(|| "    ".into());
        let new_s = new.map(|n| pad(n, 4)).unwrap_or_else(|| "    ".into());
        let old_active = line.kind == Kind::Deletion;
        let new_active = line.kind == Kind::Addition;
        // Each side of the gutter gets the same background as its line
        // type (deletion/addition bg or the muted gutter bg), so the
        // accent color visually extends into the line-number column.
        let old_bg = if old_active {
            Some(t.del_bg)
        } else if new_active {
            Some(t.add_bg)
        } else {
            Some(t.meta_bg)
        };
        let new_bg = if new_active {
            Some(t.add_bg)
        } else if old_active {
            Some(t.del_bg)
        } else {
            Some(t.meta_bg)
        };
        out.push_str(&paint(
            &old_s,
            old_bg,
            Some(if old_active { t.del_accent } else { t.meta_fg }),
            false,
            !old_active,
        ));
        out.push_str(&paint(
            &format!(" {}", new_s),
            new_bg,
            Some(if new_active { t.add_accent } else { t.meta_fg }),
            false,
            !new_active,
        ));
        out.push_str(&paint(" ", new_bg, None, false, false));
    }

    let (prefix, bg, fg) = match line.kind {
        Kind::Addition => ("+  ", Some(t.add_bg), t.add_accent),
        Kind::Deletion => ("-  ", Some(t.del_bg), t.del_accent),
        _ => ("   ", None, t.meta_fg),
    };
    out.push_str(&paint(prefix, bg, Some(fg), false, false));

    if tokens.is_empty() {
        out.push_str(&paint(&line.text, bg, Some(fg), false, false));
    } else {
        for tok in tokens {
            out.push_str(&paint(&tok.text, bg, Some(tok.color), false, false));
        }
        out.push_str(RESET);
    }
    out
}

/// Decide whether to emit the `@@ -A,B +C,D @@` hunk header. The
/// header is suppressed when:
///
/// 1. The hunk contains no actual changes (only context lines), or
/// 2. The file is shown as a single hunk starting at line 1 in both
///    versions (the entire file is being displayed, so the header is
///    redundant noise next to the title bar).
fn should_hide_hunk_header(file: &FileDiff, hunk: &Hunk) -> bool {
    let has_change = hunk
        .lines
        .iter()
        .any(|l| matches!(l.kind, Kind::Addition | Kind::Deletion));
    if !has_change {
        return true;
    }
    if file.hunks.len() != 1 {
        return false;
    }
    if hunk.old_start != 1 || hunk.new_start != 1 {
        return false;
    }
    let old_count = hunk
        .lines
        .iter()
        .filter(|l| matches!(l.kind, Kind::Context | Kind::Deletion))
        .count();
    let new_count = hunk
        .lines
        .iter()
        .filter(|l| matches!(l.kind, Kind::Context | Kind::Addition))
        .count();
    let (declared_old_start, declared_old_count, declared_new_start, declared_new_count) =
        parse_hunk_specs(&hunk.header);
    old_count == declared_old_count
        && new_count == declared_new_count
        && declared_old_start == Some(hunk.old_start)
        && declared_new_start == Some(hunk.new_start)
}

fn parse_hunk_specs(header: &str) -> (Option<usize>, usize, Option<usize>, usize) {
    let parts: Vec<&str> = header.split_whitespace().collect();
    let old = parts.get(1).unwrap_or(&"");
    let new = parts.get(2).unwrap_or(&"");
    let (old_start, old_count) = parse_old_spec(old);
    let (new_start, new_count) = parse_new_spec(new);
    (Some(old_start), old_count, Some(new_start), new_count)
}

fn parse_old_spec(spec: &str) -> (usize, usize) {
    let trimmed = spec.trim_start_matches('-');
    let mut split = trimmed.splitn(2, ',');
    let start = split.next().unwrap_or("1").parse().unwrap_or(1);
    let count = split.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    (start, count)
}

fn parse_new_spec(spec: &str) -> (usize, usize) {
    let trimmed = spec.trim_start_matches('+');
    let mut split = trimmed.splitn(2, ',');
    let start = split.next().unwrap_or("1").parse().unwrap_or(1);
    let count = split.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    (start, count)
}
