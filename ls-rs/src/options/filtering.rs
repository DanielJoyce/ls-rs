use crate::core::filesystem::FileInfo;
use regex::Regex;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

#[derive(Debug, Default)]
pub struct FilterOptions {
    pub show_hidden: bool,
    pub pattern: Option<String>,
    pub regex: Option<Regex>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub file_types: FileTypes,
}

#[derive(Debug, Default)]
pub struct FileTypes {
    pub files: bool,
    pub dirs: bool,
    pub symlinks: bool,
    pub sockets: bool,
    pub pipes: bool,
    pub devices: bool,
}

impl FileTypes {
    pub fn all() -> Self {
        Self {
            files: true,
            dirs: true,
            symlinks: true,
            sockets: true,
            pipes: true,
            devices: true,
        }
    }
}

pub fn filter_entries(entries: Vec<FileInfo>, options: &FilterOptions) -> Vec<FileInfo> {
    entries
        .into_iter()
        .filter(|entry| should_include_entry(entry, options))
        .collect()
}

fn should_include_entry(entry: &FileInfo, options: &FilterOptions) -> bool {
    // Check hidden files
    if !options.show_hidden && entry.name.starts_with('.') {
        return false;
    }

    // Check file type
    if !matches_file_type(entry, &options.file_types) {
        return false;
    }

    // Check size constraints
    if let Some(min_size) = options.min_size {
        if entry.metadata.len() < min_size {
            return false;
        }
    }
    if let Some(max_size) = options.max_size {
        if entry.metadata.len() > max_size {
            return false;
        }
    }

    // Check pattern matching
    if let Some(pattern) = &options.pattern {
        if !Path::new(&entry.name).matches_pattern(pattern) {
            return false;
        }
    }

    // Check regex matching
    if let Some(regex) = &options.regex {
        if !regex.is_match(&entry.name) {
            return false;
        }
    }

    true
}

fn matches_file_type(entry: &FileInfo, types: &FileTypes) -> bool {
    if entry.metadata.is_file() && types.files {
        return true;
    }
    if entry.metadata.is_dir() && types.dirs {
        return true;
    }
    if entry.metadata.file_type().is_symlink() && types.symlinks {
        return true;
    }
    if entry.metadata.file_type().is_socket() && types.sockets {
        return true;
    }
    if entry.metadata.file_type().is_fifo() && types.pipes {
        return true;
    }
    if (entry.metadata.file_type().is_block_device() || entry.metadata.file_type().is_char_device())
        && types.devices
    {
        return true;
    }
    false
}

trait PatternMatching {
    fn matches_pattern(&self, pattern: &str) -> bool;
}

impl PatternMatching for Path {
    fn matches_pattern(&self, pattern: &str) -> bool {
        if let Some(name) = self.file_name() {
            if let Some(name_str) = name.to_str() {
                return pattern_match(name_str, pattern);
            }
        }
        false
    }
}

fn pattern_match(name: &str, pattern: &str) -> bool {
    glob::Pattern::new(pattern)
        .map(|p| p.matches(name))
        .unwrap_or(false)
}
