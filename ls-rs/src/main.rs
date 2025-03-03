mod core;
mod options;
mod security;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Paths to list
    #[arg(default_value = ".")]
    paths: Vec<String>,

    /// Use a long listing format
    #[arg(short = 'l', long)]
    long: bool,

    /// Show hidden files
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Sort by (name, size, time)
    #[arg(short = 's', long = "sort", value_enum, default_value = "name")]
    sort: SortBy,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// Enable colorized output
    #[arg(short = 'c', long = "color", default_value = "true")]
    color: bool,

    /// Display file sizes in human readable format
    #[arg(short = 'h', long = "human-readable")]
    human_readable: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum SortBy {
    Name,
    Size,
    Time,
}

fn main() {
    let args = Args::parse();
    let config = core::display::DisplayConfig {
        term_width: core::display::get_terminal_width(),
        color_enabled: args.color,
        use_long_format: args.long,
        human_readable: args.human_readable,
    };

    for path in &args.paths {
        if let Err(e) = core::filesystem::list_directory(path, &args, &config) {
            eprintln!("rust-ls: {}: {}", path, e);
        }
    }
}
