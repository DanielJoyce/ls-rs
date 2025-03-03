# Character Encoding Handling

## Overview
Manages character encoding conversion and display for different character sets and encodings.

## Character Processing

### Multi-byte Support
- Uses `mbrtowc()` for multi-byte conversion
- Handles wide character operations
- Manages character boundaries
- Supports state-dependent encodings

### Width Calculation
- Determines display width via `wcwidth()`
- Handles zero-width characters
- Manages combining characters
- Supports East Asian width

## Encoding Operations

### Conversion Functions
- String to wide character conversion
- Wide character to string conversion
- State management for conversions
- Error handling for invalid sequences

### Display Formatting
- Alignment of multi-byte characters
- Padding calculations
- Column width adjustment
- Terminal encoding consideration 