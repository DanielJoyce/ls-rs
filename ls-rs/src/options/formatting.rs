use std::fmt;

#[derive(Debug, Clone)]
pub struct FormattingOptions {
    pub format: ListFormat,
    pub size_format: SizeFormat,
    pub time_format: TimeFormat,
    pub color_mode: ColorMode,
    pub indicators: bool,
}

#[derive(Debug, Clone)]
pub enum ListFormat {
    OneLine,
    Columns,
    Long,
    Commas,
}

#[derive(Debug, Clone)]
pub enum SizeFormat {
    Bytes,
    Human,
    SI,
    Blocks,
}

#[derive(Debug, Clone)]
pub enum TimeFormat {
    Default,
    ISO,
    Full,
    Relative,
}

#[derive(Debug, Clone)]
pub enum ColorMode {
    Never,
    Always,
    Auto,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            format: ListFormat::Columns,
            size_format: SizeFormat::Bytes,
            time_format: TimeFormat::Default,
            color_mode: ColorMode::Auto,
            indicators: false,
        }
    }
}

pub fn format_size(size: u64, format: &SizeFormat) -> String {
    match format {
        SizeFormat::Bytes => size.to_string(),
        SizeFormat::Human => humanize_size(size, 1024),
        SizeFormat::SI => humanize_size(size, 1000),
        SizeFormat::Blocks => format!("{}", (size + 511) / 512),
    }
}

fn humanize_size(size: u64, base: u64) -> String {
    let units = if base == 1024 {
        ["B", "KiB", "MiB", "GiB", "TiB"]
    } else {
        ["B", "KB", "MB", "GB", "TB"]
    };

    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= base as f64 && unit_index < units.len() - 1 {
        size /= base as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, units[unit_index])
    } else {
        format!("{:.1} {}", size, units[unit_index])
    }
}

pub fn should_use_color(mode: &ColorMode) -> bool {
    match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    }
}
