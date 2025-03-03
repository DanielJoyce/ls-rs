use crate::core::display::DisplayConfig;
use crate::{Args, SortBy};
use std::fs::{self, DirEntry};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub struct FileInfo {
    pub name: String,
    pub metadata: fs::Metadata,
    pub path: String,
}

pub fn list_directory(path: &str, args: &Args, config: &DisplayConfig) -> io::Result<()> {
    let path = Path::new(path);
    let mut entries = collect_entries(path, args)?;

    sort_entries(&mut entries, args);

    crate::core::display::display_entries(&entries, config)
}

fn collect_entries(path: &Path, args: &Args) -> io::Result<Vec<FileInfo>> {
    let mut entries = Vec::new();
    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        // Skip hidden files unless -a flag is present
        if !args.all && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let path = entry.path().to_string_lossy().into_owned();

        entries.push(FileInfo {
            name,
            metadata,
            path,
        });
    }

    Ok(entries)
}

fn display_long_format(entries: &[FileInfo]) -> io::Result<()> {
    for entry in entries {
        println!(
            "{} {} {} {} {} {}",
            get_permissions_string(&entry.metadata),
            entry.metadata.len(),
            get_user_name(entry.metadata.uid()),
            get_group_name(entry.metadata.gid()),
            format_time(entry.metadata.modified()?),
            entry.name
        );
    }
    Ok(())
}

fn display_short_format(entries: &[FileInfo]) -> io::Result<()> {
    for entry in entries {
        println!("{}", entry.name);
    }
    Ok(())
}

// Helper functions to be implemented
fn get_permissions_string(metadata: &fs::Metadata) -> String {
    // To be implemented in security/permissions.rs
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
    chrono::DateTime::<chrono::Local>::from(time)
        .format("%b %d %H:%M")
        .to_string()
}

fn sort_entries(entries: &mut Vec<FileInfo>, args: &Args) {
    match args.sort {
        SortBy::Name => entries.sort_by(|a, b| a.name.cmp(&b.name)),
        SortBy::Size => entries.sort_by(|a, b| b.metadata.len().cmp(&a.metadata.len())),
        SortBy::Time => entries.sort_by(|a, b| {
            b.metadata
                .modified()
                .unwrap_or_else(|_| std::time::UNIX_EPOCH)
                .cmp(
                    &a.metadata
                        .modified()
                        .unwrap_or_else(|_| std::time::UNIX_EPOCH),
                )
        }),
    }

    if args.reverse {
        entries.reverse();
    }
}
