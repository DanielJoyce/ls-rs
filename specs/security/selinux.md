# SELinux Context Handling

## Overview
Manages Security-Enhanced Linux (SELinux) context information for file listings.

## Context Operations

### Context Retrieval
- Gets SELinux security contexts
- Handles context string formatting
- Manages context memory with `freecon()`
- Supports context inheritance

### Display Integration
- Shows security contexts in long format
- Handles context field width
- Manages context alignment
- Supports context coloring

## Configuration

### Context Display Options
- Enable/disable context display
- Context field formatting
- Context truncation handling
- Special context indicators

### Error Handling
- Manages missing context information
- Handles access denied scenarios
- Reports context-related errors
- Provides fallback behavior 