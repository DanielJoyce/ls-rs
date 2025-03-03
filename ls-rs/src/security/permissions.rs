use std::fs::Metadata;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};

pub fn get_mode_string(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut result = String::with_capacity(10);

    // File type
    result.push(get_file_type_char(metadata));

    // User permissions
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o100 != 0 {
        if mode & 0o4000 != 0 {
            's'
        } else {
            'x'
        }
    } else {
        if mode & 0o4000 != 0 {
            'S'
        } else {
            '-'
        }
    });

    // Group permissions
    result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o010 != 0 {
        if mode & 0o2000 != 0 {
            's'
        } else {
            'x'
        }
    } else {
        if mode & 0o2000 != 0 {
            'S'
        } else {
            '-'
        }
    });

    // Other permissions
    result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o001 != 0 {
        if mode & 0o1000 != 0 {
            't'
        } else {
            'x'
        }
    } else {
        if mode & 0o1000 != 0 {
            'T'
        } else {
            '-'
        }
    });

    result
}

fn get_file_type_char(metadata: &Metadata) -> char {
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
