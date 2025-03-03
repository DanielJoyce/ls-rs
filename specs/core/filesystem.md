# Core Filesystem Operations

## Overview
The ls utility provides comprehensive filesystem entry enumeration and attribute reading capabilities.

## Directory Operations

### Directory Traversal
- Opens directories using `opendir()`
- Reads directory entries via `readdir()`
- Closes directories with `closedir()`
- Supports recursive traversal for subdirectories
- Gets current working directory using `getcwd()`
- Handles directory file descriptors with `dirfd()`

### Path Resolution
- Resolves symbolic links using `readlink()`
- Handles both absolute and relative paths
- Supports path pattern matching via `fnmatch()`

## File Information Retrieval

### Status Information
- Uses `lstat()` for file status without following symlinks
- Uses `statx()` for extended file information
- Retrieves:
  - File type
  - Size
  - Timestamps (access, modify, change)
  - Device information
  - Inode numbers
  - Link counts

### Position Handling
- Manages file positions using `lseek()`
- Handles special files and devices via `ioctl()`

## Error Handling
- Implements robust error checking for all filesystem operations
- Reports filesystem errors with appropriate error messages
- Handles permission denied scenarios
- Manages "No such file or directory" cases 