# Display Formatting

## Overview
Handles the formatting and presentation of file listings with various output formats and styles.

## Output Formats

### Basic Listing
- Single column output
- Multiple columns with appropriate spacing
- One entry per line with additional information

### Long Format (-l)
- Detailed listing including:
  - Permissions
  - Link count
  - Owner name
  - Group name
  - Size
  - Timestamp
  - Name

### Special Formats
- Comma-separated output
- Tab-separated output
- Custom format strings
- Machine-readable output

## Text Handling

### Character Encoding
- Uses `mbrtowc()` for multi-byte character conversion
- Handles wide characters with `wcwidth()`
- Supports various character encodings
- Manages display width with `wcswidth()`

### Output Buffering
- Implements buffered output for efficiency
- Uses `fwrite_unlocked()` for thread-safe output
- Manages output streams with `fflush()`
- Handles terminal output specially

## Terminal Handling
- Detects terminal capabilities using `isatty()`
- Manages terminal control via `tcgetpgrp()`
- Adjusts output based on terminal width
- Handles color and formatting escape sequences 