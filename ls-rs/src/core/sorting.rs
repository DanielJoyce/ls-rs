use crate::core::filesystem::FileInfo;
use std::cmp::Ordering;
use std::os::unix::fs::{FileTypeExt, MetadataExt};

#[derive(Debug, Clone, Copy)]
pub enum SortKey {
    Name,        // Sort by filename
    Size,        // Sort by file size
    Time,        // Sort by modification time
    Extension,   // Sort by file extension
    Type,        // Sort by file type
    Owner,       // Sort by file owner
    Group,       // Sort by file group
    Permissions, // Sort by file permissions
    Inode,       // Sort by inode number
    None,        // No sorting
}

#[derive(Debug)]
pub struct SortOptions {
    pub key: SortKey,
    pub reverse: bool,        // Reverse the sort order
    pub dirs_first: bool,     // List directories before files
    pub case_sensitive: bool, // Use case-sensitive sorting
    pub numeric_sort: bool,   // Sort numbers numerically
    pub version_sort: bool,   // Sort version numbers
    pub locale_sort: bool,    // Use locale-based sorting
}

impl Default for SortOptions {
    fn default() -> Self {
        Self {
            key: SortKey::Name,
            reverse: false,
            dirs_first: false,
            case_sensitive: true,
            numeric_sort: false,
            version_sort: false,
            locale_sort: false,
        }
    }
}

pub fn sort_entries(entries: &mut Vec<FileInfo>, options: &SortOptions) {
    entries.sort_by(|a, b| {
        // Handle directories first if enabled
        if options.dirs_first {
            match (a.metadata.is_dir(), b.metadata.is_dir()) {
                (true, false) => return Ordering::Less,
                (false, true) => return Ordering::Greater,
                _ => {}
            }
        }

        let mut ordering = match options.key {
            SortKey::Name => compare_names(a, b, options.case_sensitive),
            SortKey::Size => compare_sizes(a, b),
            SortKey::Time => compare_times(a, b),
            SortKey::Extension => compare_extensions(a, b, options.case_sensitive),
            SortKey::Type => compare_types(a, b),
            SortKey::Owner => compare_owners(a, b),
            SortKey::Group => compare_groups(a, b),
            SortKey::Permissions => compare_permissions(a, b),
            SortKey::Inode => compare_inodes(a, b),
            SortKey::None => Ordering::Equal,
        };

        if options.reverse {
            ordering = ordering.reverse();
        }

        ordering
    });
}

fn compare_names(a: &FileInfo, b: &FileInfo, case_sensitive: bool) -> Ordering {
    if case_sensitive {
        a.name.cmp(&b.name)
    } else {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    }
}

fn compare_sizes(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.len().cmp(&b.metadata.len())
}

fn compare_times(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.modified().ok().cmp(&b.metadata.modified().ok())
}

fn compare_extensions(a: &FileInfo, b: &FileInfo, case_sensitive: bool) -> Ordering {
    let ext_a = std::path::Path::new(&a.name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let ext_b = std::path::Path::new(&b.name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if case_sensitive {
        ext_a.cmp(ext_b)
    } else {
        ext_a.to_lowercase().cmp(&ext_b.to_lowercase())
    }
}

fn get_file_type(metadata: &std::fs::Metadata) -> char {
    if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else if metadata.file_type().is_block_device() {
        'b'
    } else if metadata.file_type().is_char_device() {
        'c'
    } else if metadata.file_type().is_fifo() {
        'p'
    } else if metadata.file_type().is_socket() {
        's'
    } else {
        '-'
    }
}

fn compare_types(a: &FileInfo, b: &FileInfo) -> Ordering {
    let type_a = get_file_type(&a.metadata);
    let type_b = get_file_type(&b.metadata);
    type_a.cmp(&type_b)
}

fn compare_owners(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.uid().cmp(&b.metadata.uid())
}

fn compare_groups(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.gid().cmp(&b.metadata.gid())
}

fn compare_permissions(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.mode().cmp(&b.metadata.mode())
}

fn compare_inodes(a: &FileInfo, b: &FileInfo) -> Ordering {
    a.metadata.ino().cmp(&b.metadata.ino())
}
