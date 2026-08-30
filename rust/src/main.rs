use clap::Parser;
use diffview::{render, RenderOptions};
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(
    name = "diffview",
    version = "0.2.0",
    about = "Fast syntax-highlighted terminal diff renderer"
)]
struct Args {
    #[arg(long, alias = "shiki-theme", default_value = "github-dark-default")]
    syntax_theme: String,
    #[arg(long, default_value = "auto", value_parser = ["dark", "light", "auto"])]
    theme: String,
    #[arg(long)]
    no_line_numbers: bool,
    #[arg(long, help = "Highlight context lines too (default: changed lines only)")]
    full: bool,
}

fn main() {
    let args = Args::parse();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    if input.is_empty() {
        return;
    }
    let width: usize = std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v: &usize| *v > 0)
        .unwrap_or(diffview::DEFAULT_WIDTH);
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
