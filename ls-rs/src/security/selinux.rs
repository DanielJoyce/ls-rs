use selinux::{self, KernelSupport, SecurityContext};
use std::ffi::{CStr, CString};
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SELinuxContext {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub range: String,
}

pub struct SELinuxConfig {
    pub enabled: bool,
    pub show_context: bool,
    pub truncate_context: bool,
    pub max_context_width: Option<usize>,
}

impl Default for SELinuxConfig {
    fn default() -> Self {
        Self {
            enabled: selinux::kernel_support() != KernelSupport::Unsupported,
            show_context: false,
            truncate_context: false,
            max_context_width: None,
        }
    }
}

pub struct SELinuxHandler {
    config: SELinuxConfig,
}

impl SELinuxHandler {
    pub fn new(config: SELinuxConfig) -> Self {
        Self { config }
    }

    /// Get the SELinux context for a file
    pub fn get_context(&self, path: &Path) -> io::Result<Option<SELinuxContext>> {
        if !self.config.enabled {
            return Ok(None);
        }

        match SecurityContext::of_path(path, true, true) {
            Ok(Some(context)) => match context.to_c_string() {
                Ok(Some(context_str)) => self
                    .parse_context(&context_str)
                    .map(Some)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
                Ok(None) => Ok(None),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            },
            Ok(None) => Ok(None),
            Err(_) => Ok(None),
        }
    }

    /// Format the context string for display
    pub fn format_context(&self, context: &SELinuxContext) -> String {
        let full_context = format!(
            "{}:{}:{}:{}",
            context.user, context.role, context.type_, context.range
        );

        if let Some(max_width) = self.config.max_context_width {
            if self.config.truncate_context && full_context.len() > max_width {
                format!("{}...", &full_context[..max_width.saturating_sub(3)])
            } else {
                full_context
            }
        } else {
            full_context
        }
    }

    /// Parse a context string into its components
    fn parse_context(&self, context_str: &CStr) -> Result<SELinuxContext, &'static str> {
        let context_str = context_str.to_string_lossy();
        let parts: Vec<&str> = context_str.split(':').collect();
        if parts.len() != 4 {
            return Err("Invalid SELinux context format");
        }

        Ok(SELinuxContext {
            user: parts[0].to_string(),
            role: parts[1].to_string(),
            type_: parts[2].to_string(),
            range: parts[3].to_string(),
        })
    }

    /// Check if a path has a valid SELinux context
    pub fn has_context(&self, path: &Path) -> bool {
        if !self.config.enabled {
            return false;
        }

        SecurityContext::of_path(path, true, true).is_ok()
    }

    /// Get the context field width for alignment
    pub fn get_context_width(&self, contexts: &[Option<SELinuxContext>]) -> usize {
        if !self.config.enabled || !self.config.show_context {
            return 0;
        }

        contexts
            .iter()
            .filter_map(|c| c.as_ref())
            .map(|c| self.format_context(c).len())
            .max()
            .unwrap_or(0)
    }
}

/// Error type for SELinux operations
#[derive(Debug)]
pub enum SELinuxError {
    Disabled,
    NotFound,
    AccessDenied,
    InvalidContext,
    Other(String),
}

impl std::fmt::Display for SELinuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SELinuxError::Disabled => write!(f, "SELinux is disabled"),
            SELinuxError::NotFound => write!(f, "SELinux context not found"),
            SELinuxError::AccessDenied => write!(f, "Access denied to SELinux context"),
            SELinuxError::InvalidContext => write!(f, "Invalid SELinux context"),
            SELinuxError::Other(msg) => write!(f, "SELinux error: {}", msg),
        }
    }
}

impl std::error::Error for SELinuxError {}

// Integration with the FileInfo struct
pub trait SELinuxExt {
    fn get_selinux_context(&self, handler: &SELinuxHandler) -> io::Result<Option<SELinuxContext>>;
}

impl SELinuxExt for crate::core::filesystem::FileInfo {
    fn get_selinux_context(&self, handler: &SELinuxHandler) -> io::Result<Option<SELinuxContext>> {
        handler.get_context(Path::new(&self.path))
    }
}
