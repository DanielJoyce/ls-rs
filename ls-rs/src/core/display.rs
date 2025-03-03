use crate::core::filesystem::FileInfo;
use std::io::{self, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use unicode_width::UnicodeWidthStr;

pub struct DisplayConfig {
    pub term_width: usize,
    pub color_enabled: bool,
    pub use_long_format: bool,
    pub human_readable: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            term_width: get_terminal_width(),
            color_enabled: true,
            use_long_format: false,
            human_readable: false,
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

    for entry in entries {
        let color = if config.color_enabled {
            get_entry_color(entry)
        } else {
            ""
        };

        writeln!(
            handle,
            "{} {:>width_links$} {} {} {:>width_size$} {} {}{}{}",
            get_mode_string(&entry.metadata),
            entry.metadata.nlink(),
            get_user_name(entry.metadata.uid()),
            get_group_name(entry.metadata.gid()),
            entry.metadata.len(),
            format_time(entry.metadata.modified()?),
            color,
            entry.name,
            if config.color_enabled { "\x1b[0m" } else { "" },
            width_links = max_links_width,
            width_size = max_size_width,
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
    // This will be implemented in security/permissions.rs
    String::new()
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
