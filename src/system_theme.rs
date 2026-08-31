use crate::{ansi_color, Syntax, Theme, DARK, DEFAULT_COLOR, LIGHT};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::time::{Duration, Instant};

const ANSI_FALLBACK: [u32; 16] = [
    0x000000, 0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xc0c0c0, 0x808080,
    0xff0000, 0x00ff00, 0xffff00, 0x0000ff, 0xff00ff, 0x00ffff, 0xffffff,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TerminalPalette {
    foreground: u32,
    background: u32,
    ansi: [u32; 16],
}

pub fn detect(light: bool) -> Option<Theme> {
    let term = std::env::var("TERM").ok();
    let in_lazygit = std::env::var_os("LAZYGIT_COLUMNS").is_some();
    if !allows_color(term.as_deref(), in_lazygit) {
        return None;
    }
    if term.as_deref() == Some("dumb") {
        return Some(fallback_theme(light));
    }
    if unsafe { libc::isatty(libc::STDOUT_FILENO) } == 1 {
        if let Some(palette) = query_palette() {
            return Some(generate(palette));
        }
    }
    Some(fallback_theme(light))
}

fn allows_color(term: Option<&str>, in_lazygit: bool) -> bool {
    term != Some("dumb") || in_lazygit
}

fn query_palette() -> Option<TerminalPalette> {
    let mut tty = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .ok()?;
    let fd = tty.as_raw_fd();
    let mut original = unsafe { std::mem::zeroed::<libc::termios>() };
    if unsafe { libc::tcgetattr(fd, &mut original) } != 0 {
        return None;
    }
    let mut raw = original;
    unsafe { libc::cfmakeraw(&mut raw) };
    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw) } != 0 {
        return None;
    }
    let _guard = TermiosGuard { fd, original };

    let mut query = String::from("\x1b]10;?\x1b\\\x1b]11;?\x1b\\");
    for index in 0..16 {
        query.push_str(&format!("\x1b]4;{index};?\x1b\\"));
    }
    tty.write_all(query.as_bytes()).ok()?;
    tty.flush().ok()?;

    let deadline = Instant::now() + Duration::from_millis(250);
    let mut response = Vec::new();
    let mut chunk = [0u8; 4096];
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let mut pollfd = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let timeout = remaining.as_millis().clamp(1, i32::MAX as u128) as i32;
        if unsafe { libc::poll(&mut pollfd, 1, timeout) } <= 0 {
            break;
        }
        match tty.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => response.extend_from_slice(&chunk[..read]),
        }
        if parse_responses(&response).is_some_and(|palette| {
            palette.foreground.is_some()
                && palette.background.is_some()
                && palette.ansi.iter().all(Option::is_some)
        }) {
            break;
        }
    }

    let parsed = parse_responses(&response)?;
    let mut ansi = ANSI_FALLBACK;
    for (index, color) in parsed.ansi.into_iter().enumerate() {
        if let Some(color) = color {
            ansi[index] = color;
        }
    }
    Some(TerminalPalette {
        foreground: parsed.foreground?,
        background: parsed.background?,
        ansi,
    })
}

fn fallback_theme(light: bool) -> Theme {
    let base = if light { LIGHT } else { DARK };
    let default = DEFAULT_COLOR;
    let muted = ansi_color(8);
    let red = ansi_color(1);
    let green = ansi_color(2);
    let yellow = ansi_color(3);
    let blue = ansi_color(4);
    let magenta = ansi_color(5);
    let cyan = ansi_color(6);

    Theme {
        meta_bg: base.meta_bg,
        meta_fg: muted,
        hunk_bg: base.hunk_bg,
        hunk_fg: base.hunk_fg,
        hunk_gutter_bg: base.hunk_gutter_bg,
        header_bg: base.header_bg,
        header_fg: default,
        header_muted: muted,
        separator: muted,
        add_bg: base.add_bg,
        del_bg: base.del_bg,
        add_gutter_bg: base.add_gutter_bg,
        del_gutter_bg: base.del_gutter_bg,
        add_accent: green,
        del_accent: red,
        syntax: Syntax {
            comment: muted,
            keyword: magenta,
            string: green,
            string_special: cyan,
            number: yellow,
            constant_builtin: red,
            function: blue,
            function_method: blue,
            function_macro: blue,
            type_: cyan,
            type_builtin: cyan,
            constructor: cyan,
            variable: default,
            variable_builtin: red,
            variable_parameter: default,
            variable_member: default,
            property: default,
            module: cyan,
            operator: cyan,
            tag: red,
            attribute: yellow,
            label: cyan,
            punctuation: default,
            default,
        },
    }
}

struct TermiosGuard {
    fd: libc::c_int,
    original: libc::termios,
}

impl Drop for TermiosGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
        }
    }
}

#[derive(Clone, Copy)]
struct ParsedResponses {
    foreground: Option<u32>,
    background: Option<u32>,
    ansi: [Option<u32>; 16],
}

fn parse_responses(bytes: &[u8]) -> Option<ParsedResponses> {
    let mut parsed = ParsedResponses {
        foreground: None,
        background: None,
        ansi: [None; 16],
    };
    let mut found = false;
    let mut cursor = 0;
    while cursor + 2 <= bytes.len() {
        let Some(start) = bytes[cursor..].windows(2).position(|part| part == b"\x1b]") else {
            break;
        };
        let payload_start = cursor + start + 2;
        let Some((payload_end, terminator_len)) = osc_end(bytes, payload_start) else {
            break;
        };
        let payload = String::from_utf8_lossy(&bytes[payload_start..payload_end]);
        let parts: Vec<&str> = payload.split(';').collect();
        match parts.as_slice() {
            ["10", color] => {
                parsed.foreground = parse_color(color);
                found |= parsed.foreground.is_some();
            }
            ["11", color] => {
                parsed.background = parse_color(color);
                found |= parsed.background.is_some();
            }
            ["4", rest @ ..] => {
                for pair in rest.chunks_exact(2) {
                    if let (Ok(index), Some(color)) =
                        (pair[0].parse::<usize>(), parse_color(pair[1]))
                    {
                        if index < parsed.ansi.len() {
                            parsed.ansi[index] = Some(color);
                            found = true;
                        }
                    }
                }
            }
            _ => {}
        }
        cursor = payload_end + terminator_len;
    }
    found.then_some(parsed)
}

fn osc_end(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    for index in start..bytes.len() {
        if bytes[index] == 0x07 {
            return Some((index, 1));
        }
        if bytes[index] == 0x1b && bytes.get(index + 1) == Some(&b'\\') {
            return Some((index, 2));
        }
    }
    None
}

fn parse_color(value: &str) -> Option<u32> {
    let components = value
        .strip_prefix("rgb:")?
        .split('/')
        .take(3)
        .map(scale_component)
        .collect::<Option<Vec<_>>>()?;
    if components.len() != 3 {
        return None;
    }
    Some((components[0] << 16) | (components[1] << 8) | components[2])
}

fn scale_component(value: &str) -> Option<u32> {
    if value.is_empty() || value.len() > 4 {
        return None;
    }
    let component = u32::from_str_radix(value, 16).ok()?;
    let max = (1u32 << (value.len() * 4)) - 1;
    Some((component * 255 + max / 2) / max)
}

fn generate(palette: TerminalPalette) -> Theme {
    let bg = palette.background;
    let fg = palette.foreground;
    let dark = luminance(bg) < luminance(fg);
    let grays = gray_scale(bg, dark);
    let muted = muted_text(bg, dark);
    let red = palette.ansi[1];
    let green = palette.ansi[2];
    let yellow = palette.ansi[3];
    let blue = palette.ansi[4];
    let magenta = palette.ansi[5];
    let cyan = palette.ansi[6];
    let alpha = if dark { 0.22 } else { 0.14 };

    Theme {
        meta_bg: grays[2],
        meta_fg: muted,
        hunk_bg: grays[2],
        hunk_fg: grays[7],
        hunk_gutter_bg: grays[3],
        header_bg: grays[2],
        header_fg: fg,
        header_muted: muted,
        separator: grays[6],
        add_bg: tint(bg, green, alpha),
        del_bg: tint(bg, red, alpha),
        add_gutter_bg: tint(grays[2], green, alpha),
        del_gutter_bg: tint(grays[2], red, alpha),
        add_accent: green,
        del_accent: red,
        syntax: Syntax {
            comment: muted,
            keyword: magenta,
            string: green,
            string_special: cyan,
            number: yellow,
            constant_builtin: red,
            function: blue,
            function_method: blue,
            function_macro: blue,
            type_: cyan,
            type_builtin: cyan,
            constructor: cyan,
            variable: fg,
            variable_builtin: red,
            variable_parameter: fg,
            variable_member: fg,
            property: fg,
            module: cyan,
            operator: cyan,
            tag: red,
            attribute: yellow,
            label: cyan,
            punctuation: fg,
            default: fg,
        },
    }
}

fn channels(color: u32) -> (f64, f64, f64) {
    (
        ((color >> 16) & 0xff) as f64,
        ((color >> 8) & 0xff) as f64,
        (color & 0xff) as f64,
    )
}

fn rgb(red: f64, green: f64, blue: f64) -> u32 {
    ((red.round().clamp(0.0, 255.0) as u32) << 16)
        | ((green.round().clamp(0.0, 255.0) as u32) << 8)
        | blue.round().clamp(0.0, 255.0) as u32
}

fn luminance(color: u32) -> f64 {
    let (red, green, blue) = channels(color);
    0.299 * red + 0.587 * green + 0.114 * blue
}

fn tint(base: u32, overlay: u32, alpha: f64) -> u32 {
    let (base_r, base_g, base_b) = channels(base);
    let (over_r, over_g, over_b) = channels(overlay);
    rgb(
        base_r + (over_r - base_r) * alpha,
        base_g + (over_g - base_g) * alpha,
        base_b + (over_b - base_b) * alpha,
    )
}

fn gray_scale(background: u32, dark: bool) -> [u32; 13] {
    let (bg_r, bg_g, bg_b) = channels(background);
    let lum = luminance(background);
    let mut grays = [background; 13];
    for (index, gray) in grays.iter_mut().enumerate().skip(1) {
        let factor = index as f64 / 12.0;
        *gray = if dark {
            if lum < 10.0 {
                let value = factor * 0.4 * 255.0;
                rgb(value, value, value)
            } else {
                let ratio = (lum + (255.0 - lum) * factor * 0.4) / lum;
                rgb(bg_r * ratio, bg_g * ratio, bg_b * ratio)
            }
        } else if lum > 245.0 {
            let value = 255.0 - factor * 0.4 * 255.0;
            rgb(value, value, value)
        } else {
            let ratio = (lum * (1.0 - factor * 0.4)) / lum;
            rgb(bg_r * ratio, bg_g * ratio, bg_b * ratio)
        };
    }
    grays
}

fn muted_text(background: u32, dark: bool) -> u32 {
    let lum = luminance(background);
    let value = if dark {
        if lum < 10.0 {
            180.0
        } else {
            (160.0 + lum * 0.3).min(200.0)
        }
    } else if lum > 245.0 {
        75.0
    } else {
        (100.0 - (255.0 - lum) * 0.2).max(60.0)
    };
    rgb(value, value, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_terminal_color_responses() {
        let responses = b"\x1b]10;rgb:dddd/eeee/ffff\x1b\\\x1b]11;rgb:1111/2222/3333\x07\x1b]4;1;rgb:aa/00/11;2;rgb:22/bb/33\x1b\\";
        let parsed = parse_responses(responses).unwrap();
        assert_eq!(parsed.foreground, Some(0xddeeff));
        assert_eq!(parsed.background, Some(0x112233));
        assert_eq!(parsed.ansi[1], Some(0xaa0011));
        assert_eq!(parsed.ansi[2], Some(0x22bb33));
    }

    #[test]
    fn generated_theme_uses_terminal_palette() {
        let mut ansi = ANSI_FALLBACK;
        ansi[1] = 0xcc5555;
        ansi[2] = 0x55cc77;
        ansi[3] = 0xddaa44;
        ansi[4] = 0x6699dd;
        ansi[5] = 0xbb77cc;
        ansi[6] = 0x55bbbb;
        let theme = generate(TerminalPalette {
            foreground: 0xd8dee9,
            background: 0x20242c,
            ansi,
        });
        assert_eq!(theme.header_fg, 0xd8dee9);
        assert_eq!(theme.add_accent, 0x55cc77);
        assert_eq!(theme.del_accent, 0xcc5555);
        assert_eq!(theme.syntax.keyword, 0xbb77cc);
        assert_ne!(theme.add_bg, 0x20242c);
        assert_ne!(theme.del_bg, 0x20242c);
    }

    #[test]
    fn generated_theme_adapts_to_light_backgrounds() {
        let theme = generate(TerminalPalette {
            foreground: 0x202020,
            background: 0xf4f1ea,
            ansi: ANSI_FALLBACK,
        });
        assert!(luminance(theme.meta_bg) < luminance(0xf4f1ea));
        assert_eq!(theme.syntax.default, 0x202020);
    }

    #[test]
    fn fallback_theme_uses_dark_backgrounds_and_terminal_colors() {
        let theme = fallback_theme(false);
        assert_eq!(theme.header_bg, DARK.header_bg);
        assert_eq!(theme.hunk_bg, DARK.hunk_bg);
        assert_eq!(theme.add_bg, DARK.add_bg);
        assert_eq!(theme.del_bg, DARK.del_bg);
        assert_eq!(theme.header_fg, DEFAULT_COLOR);
        assert_eq!(theme.add_accent, ansi_color(2));
        assert_eq!(theme.del_accent, ansi_color(1));
        assert_eq!(theme.syntax.keyword, ansi_color(5));
        assert_eq!(theme.syntax.default, DEFAULT_COLOR);
    }

    #[test]
    fn fallback_theme_uses_light_backgrounds() {
        let theme = fallback_theme(true);
        assert_eq!(theme.header_bg, LIGHT.header_bg);
        assert_eq!(theme.hunk_bg, LIGHT.hunk_bg);
        assert_eq!(theme.add_bg, LIGHT.add_bg);
        assert_eq!(theme.del_bg, LIGHT.del_bg);
    }

    #[test]
    fn lazygit_pty_allows_colors_with_dumb_term() {
        assert!(allows_color(Some("dumb"), true));
        assert!(!allows_color(Some("dumb"), false));
    }

    #[test]
    fn paint_emits_ansi_and_terminal_default_codes() {
        let ansi = crate::paint(
            "text",
            Some(ansi_color(2)),
            Some(ansi_color(5)),
            false,
            false,
        );
        assert!(ansi.contains("48;5;2"));
        assert!(ansi.contains("38;5;5"));
        let default = crate::paint(
            "text",
            Some(DEFAULT_COLOR),
            Some(DEFAULT_COLOR),
            false,
            false,
        );
        assert!(default.contains("49;39"));
    }
}
