# Permission Handling

## Overview
Manages and displays file permissions, ownership, and access control information.

## Permission Processing

### Mode Interpretation
- Reads file mode bits
- Interprets:
  - User permissions (read/write/execute)
  - Group permissions (read/write/execute)
  - Others permissions (read/write/execute)
  - Special bits (setuid/setgid/sticky)

### Ownership Information
- Retrieves user information via `getpwuid()`
- Gets group information using `getgrgid()`
- Handles numeric and symbolic IDs
- Caches lookups for efficiency

## Display Formatting

### Symbolic Notation
- Converts mode bits to rwx notation
- Handles special permission indicators
- Supports ACL indicators
- Shows extended attribute markers

### Numeric Notation
- Octal mode representation
- Special bits display
- Permission bit masking
- Default permission handling

## Access Control

### Special Permissions
- Setuid bit handling
- Setgid bit handling
- Sticky bit handling
- Directory permission specifics

### Extended Attributes
- ACL presence indication
- Security context markers
- Special file type indicators 