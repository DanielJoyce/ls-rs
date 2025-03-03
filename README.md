# ls-rs: A Rust Implementation of ls

This project is a Rust implementation of the Unix `ls` command, created through a unique reverse
engineering and specification-driven development process.

---

Start Non-generated comments

For more information on the process, see the [blog post](https://ghuntley.com/z80/).

I use cursor AI and Sonnet 3.5 to generate almost all ( 99.9%) of the code in this project. I also
used it to generate this file.

**I have NOT undertaken any heavy checking of the claims made in this file.**

Some things may or may not be fully integrated or generated yet. I was surpised how far sonnet 3.5
could get even with the pretty lean specs it generated.

The code generated was generally syntactically crate though sometimes a crate import would be
missing.

Anyways, in the end, 1000 lines of code was generated in about 45 minutes of back and forth.

End Non-generated comments

---

## Project Genesis

The implementation was created through the following steps:

1. **Binary Analysis**: Started by analyzing the objdump of the original GNU ls command:
   - Examined the ELF64 x86-64 binary structure
   - Identified key functions and their relationships
   - Mapped out core functionality patterns

2. **Specification Generation**: Created detailed specifications organized by functional areas:

   ```
   specs/
   ├── core/
   │   ├── filesystem.md     # Core filesystem operations
   │   ├── display.md        # Output formatting and display
   │   └── sorting.md        # Entry sorting capabilities
   ├── security/
   │   ├── permissions.md    # Permission handling
   │   └── selinux.md       # SELinux context handling
   ├── i18n/
   │   ├── localization.md  # Internationalization support
   │   └── encoding.md      # Character encoding handling
   └── options/
       ├── formatting.md     # Display formatting options
       └── filtering.md      # Entry filtering options
   ```

3. **Implementation**: Systematically implemented each specification in Rust:
   - Used idiomatic Rust practices
   - Leveraged appropriate crates for core functionality
   - Maintained security and performance considerations

## Project Structure

```
ls-rs/
├── src/
│   ├── main.rs
│   ├── core/
│   │   ├── filesystem.rs
│   │   ├── display.rs
│   │   └── sorting.rs
│   ├── security/
│   │   ├── permissions.rs
│   │   └── selinux.rs
│   └── options/
│       ├── formatting.rs
│       └── filtering.rs
specs/
    └── [specification files]
```

## Key Features

- Full filesystem entry enumeration
- Configurable display formats
- File permission handling
- SELinux context support
- Sorting with multiple keys
- Unicode-aware display
- Color support
- Filtering capabilities

## Dependencies

Core dependencies were chosen carefully to maintain balance between functionality and bloat:

- `clap`: Command line argument parsing
- `libc`: System bindings
- `users`: User/group information
- `chrono`: Time handling
- `unicode-width`: Unicode character width
- `selinux`: SELinux context handling
- `termcolor`: Terminal color support

## Building

```bash
cargo build --release
```

## Usage

```bash
ls-rs [OPTIONS] [PATHS]...

Options:
    -l, --long                Use long listing format
    -a, --all                Show hidden files
    --color [WHEN]           Use colors in output
    -r, --reverse            Reverse sort order
    -S                       Sort by size
    -t                       Sort by modification time
```

## Development Process

This project demonstrates an interesting approach to software reimplementation:

1. **Analysis**: Using `objdump` to understand the original implementation
2. **Specification**: Creating detailed functional specifications
3. **Implementation**: Systematic implementation in Rust

This process allowed us to:

- Understand the core functionality deeply
- Create clean, well-organized specifications
- Implement features systematically
- Maintain compatibility while using modern Rust practices

## License

See [LICENSE](ls-rs/LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
