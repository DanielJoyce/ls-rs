use crate::core::filesystem::FileInfo;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum SortKey {
    Name,
    Size,
    Time,
    Extension,
    None,
}

#[derive(Debug)]
pub struct SortOptions {
    pub key: SortKey,
    pub reverse: bool,
    pub dirs_first: bool,
    pub case_sensitive: bool,
}

impl Default for SortOptions {
    fn default() -> Self {
        Self {
            key: SortKey::Name,
            reverse: false,
            dirs_first: false,
            case_sensitive: true,
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
