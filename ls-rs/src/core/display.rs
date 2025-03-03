use crate::core::filesystem::FileInfo;
use crate::security::permissions;
use crate::security::selinux::{SELinuxContext, SELinuxExt, SELinuxHandler};
use std::io::{self, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use unicode_width::UnicodeWidthStr;

pub struct DisplayConfig {
    pub term_width: usize,
    pub color_enabled: bool,
    pub use_long_format: bool,
    pub human_readable: bool,
    pub selinux_handler: Option<SELinuxHandler>,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            term_width: get_terminal_width(),
            color_enabled: true,
            use_long_format: false,
            human_readable: false,
            selinux_handler: None,
        }
    }
}

pub fn display_entries(entries: &[FileInfo], config: &DisplayConfig) -> io::Result<()> {
    if config.use_long_format {
        display_long_format(entries, config)
    } else {
        display_columns(entries, config)
    }
}

fn display_long_format(entries: &[FileInfo], config: &DisplayConfig) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Get SELinux contexts if enabled
    let contexts: Vec<Option<SELinuxContext>> = if let Some(handler) = &config.selinux_handler {
        entries
            .iter()
            .map(|e| e.get_selinux_context(handler))
            .collect::<io::Result<Vec<_>>>()?
    } else {
        vec![None; entries.len()]
    };

    // Calculate context width if SELinux is enabled
    let context_width = if let Some(handler) = &config.selinux_handler {
        handler.get_context_width(&contexts)
    } else {
        0
    };

    // Calculate field widths
    let max_size_width = entries
        .iter()
        .map(|e| e.metadata.len().to_string().len())
        .max()
        .unwrap_or(0);

    let max_links_width = entries
        .iter()
        .map(|e| e.metadata.nlink().to_string().len())
        .max()
        .unwrap_or(0);

    for (entry, context) in entries.iter().zip(contexts.iter()) {
        let context_str = if let (Some(handler), Some(ctx)) = (&config.selinux_handler, context) {
            format!(
                " {:<width$}",
                handler.format_context(ctx),
                width = context_width
            )
        } else {
            String::new()
        };

        writeln!(
            handle,
            "{} {:>width$} {} {} {:>size_width$} {} {}{}{}{} {}",
            get_mode_string(&entry.metadata),
            entry.metadata.nlink(),
            get_user_name(entry.metadata.uid()),
            get_group_name(entry.metadata.gid()),
            format_size(entry.metadata.len(), config.human_readable),
            format_time(entry.metadata.modified()?),
            if config.color_enabled {
                get_entry_color(entry)
            } else {
                ""
            },
            entry.name,
            if config.color_enabled { "\x1b[0m" } else { "" },
            context_str,
            if entry.metadata.file_type().is_symlink() {
                format!(" -> {}", read_link_target(entry))
            } else {
                String::new()
            },
            width = max_links_width,
            size_width = max_size_width,
        )?;
    }

    Ok(())
}

fn display_columns(entries: &[FileInfo], config: &DisplayConfig) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Calculate maximum filename width
    let max_width = entries.iter().map(|e| e.name.width()).max().unwrap_or(0);

    // Calculate number of columns that will fit
    let column_width = max_width + 2; // Add 2 for spacing
    let num_columns = std::cmp::max(1, config.term_width / column_width);
    let num_rows = (entries.len() + num_columns - 1) / num_columns;

    for row in 0..num_rows {
        for col in 0..num_columns {
            let index = col * num_rows + row;
            if index >= entries.len() {
                break;
            }

            let entry = &entries[index];
            let color = if config.color_enabled {
                get_entry_color(entry)
            } else {
                ""
            };

            if col == num_columns - 1 {
                writeln!(
                    handle,
                    "{}{}{}",
                    color,
                    entry.name,
                    if config.color_enabled { "\x1b[0m" } else { "" }
                )?;
            } else {
                write!(
                    handle,
                    "{}{}{:<width$}",
                    color,
                    entry.name,
                    if config.color_enabled { "\x1b[0m" } else { "" },
                    width = column_width
                )?;
            }
        }
    }

    Ok(())
}

pub fn get_terminal_width() -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width
    } else {
        80 // Default terminal width
    }
}

fn get_entry_color(entry: &FileInfo) -> &'static str {
    if entry.metadata.is_dir() {
        "\x1b[34m" // Blue for directories
    } else if entry.metadata.permissions().mode() & 0o111 != 0 {
        "\x1b[32m" // Green for executables
    } else {
        "\x1b[0m" // Default color
    }
}

// These functions are used by both display formats
fn get_mode_string(metadata: &std::fs::Metadata) -> String {
    permissions::get_mode_string(metadata)
}

fn get_user_name(uid: u32) -> String {
    users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string())
}

fn get_group_name(gid: u32) -> String {
    users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string())
}

fn format_time(time: std::time::SystemTime) -> String {
    use chrono::{DateTime, Local};
    let datetime: DateTime<Local> = time.into();
    datetime.format("%b %d %H:%M").to_string()
}

fn read_link_target(entry: &FileInfo) -> String {
    std::fs::read_link(Path::new(&entry.path))
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| String::from("???"))
}

fn format_size(size: u64, human_readable: bool) -> String {
    if !human_readable {
        return size.to_string();
    }

    let units = ["B", "K", "M", "G", "T", "P"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}", size as u64)
    } else {
        format!("{:.1}{}", size, units[unit_index])
    }
}

impl DisplayConfig {
    pub fn with_selinux(mut self, selinux_handler: Option<SELinuxHandler>) -> Self {
        self.selinux_handler = selinux_handler;
        self
    }
}
