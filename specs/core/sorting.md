# Entry Sorting

## Overview
Provides various sorting capabilities for directory entries based on different attributes and ordering methods.

## Sorting Methods

### Name-based Sorting
- Lexicographical sorting by name
- Case-sensitive and case-insensitive options
- Locale-aware sorting using `strcoll()`
- Extension-based sorting

### Time-based Sorting
- Modification time (`mtime`)
- Access time (`atime`)
- Creation time (`ctime`)
- Supports both forward and reverse chronological order

### Size-based Sorting
- Ascending and descending size ordering
- Handles special files and directories
- Block size considerations

## Comparison Functions

### String Comparison
- Uses `strcmp()` for basic comparison
- `strcoll()` for locale-aware comparison
- `strncmp()` for partial comparisons

### Time Comparison
- Handles various time formats
- Uses `localtime_r()` and `gmtime_r()`
- Supports timezone considerations via `tzset()`

## Sort Configuration
- Reversible sort order
- Multiple sort keys
- Directory grouping options
- Special entry handling (., ..) 