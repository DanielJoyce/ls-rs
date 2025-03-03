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

    /// Sort by (name, size, time, extension, type, owner, group)
    #[arg(short = 's', long = "sort", value_enum, default_value = "name")]
    sort: SortBy,

    /// Sort directories before files
    #[arg(short = 'd', long = "dirs-first")]
    dirs_first: bool,

    /// Use case-sensitive sorting
    #[arg(long = "case-sensitive")]
    case_sensitive: bool,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    /// Enable colorized output
    #[arg(short = 'c', long = "color", default_value = "true")]
    color: bool,

    /// Display file sizes in human readable format
    #[arg(short = 'h', long = "human-readable")]
    human_readable: bool,

    /// Show SELinux security context
    #[arg(short = 'Z', long = "context")]
    selinux: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum SortBy {
    Name,
    Size,
    Time,
    Extension,
    Type,
    Owner,
    Group,
    None,
}

fn main() {
    let args = Args::parse();
    let selinux_config = security::selinux::SELinuxConfig {
        enabled: true,
        show_context: args.selinux, // Use the selinux flag here
        truncate_context: false,
        max_context_width: None,
    };

    let config = core::display::DisplayConfig {
        term_width: core::display::get_terminal_width(),
        color_enabled: args.color,
        use_long_format: args.long,
        human_readable: args.human_readable,
        selinux_handler: if args.selinux {
            // Only create handler if SELinux is enabled
            Some(security::selinux::SELinuxHandler::new(selinux_config))
        } else {
            None
        },
    };

    for path in &args.paths {
        if let Err(e) = core::filesystem::list_directory(path, &args, &config) {
            eprintln!("rust-ls: {}: {}", path, e);
        }
    }
}
