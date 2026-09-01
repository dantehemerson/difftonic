use clap::Parser;
use difftonic::{render, RenderOptions};
use std::io::{self, Read};
use terminal_size::{terminal_size, Width};

#[derive(Parser, Debug)]
#[command(
    name = "difftonic",
    version,
    about = "Fast syntax-highlighted terminal diff renderer"
)]
struct Args {
    #[arg(long, alias = "shiki-theme", default_value = "github-dark-default")]
    syntax_theme: String,
    #[arg(long, default_value = "auto", value_parser = ["dark", "light", "auto", "system", "adaptive"])]
    theme: String,
    #[arg(long)]
    no_line_numbers: bool,
    #[arg(
        long,
        help = "Highlight context lines too (default: changed lines only)"
    )]
    full: bool,
    #[arg(long, short = 'w', help = "Width for title bar and layout")]
    width: Option<usize>,
}

fn tty_columns() -> Option<usize> {
    use std::fs::File;
    use std::os::unix::io::AsRawFd;

    let file = File::open("/dev/tty").ok()?;
    let fd = file.as_raw_fd();
    let mut winsize: libc::winsize = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut winsize) };
    if result == 0 && winsize.ws_col > 0 {
        Some(winsize.ws_col as usize)
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    if input.is_empty() {
        return;
    }
    let width: usize = args
        .width
        .filter(|v| *v > 0)
        .or_else(|| {
            std::env::var("COLUMNS")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|v: &usize| *v > 0)
        })
        .or_else(|| terminal_size().map(|(Width(w), _)| w as usize))
        .or_else(tty_columns)
        .unwrap_or(difftonic::DEFAULT_WIDTH);
    let options = RenderOptions {
        syntax_theme: args.syntax_theme,
        theme: args.theme,
        no_line_numbers: args.no_line_numbers,
        full: args.full,
        width,
    };
    let output = render(&input, &options);
    if !output.is_empty() {
        print!("{}", output);
    }
}
