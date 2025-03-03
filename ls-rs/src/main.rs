mod core;
mod options;
mod security;

use clap::Parser;

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
}

fn main() {
    let args = Args::parse();

    for path in &args.paths {
        if let Err(e) = core::filesystem::list_directory(path, &args) {
            eprintln!("rust-ls: {}: {}", path, e);
        }
    }
}
