//! Claude binary detection module - Cross-platform interface
//! 
//! This module provides a unified interface for Claude binary detection across different platforms.
//! It automatically selects the appropriate platform-specific implementation.

#[cfg(target_os = "windows")]
pub use crate::claude_binary::*;

#[cfg(not(target_os = "windows"))]
pub use crate::claude_binary_unix::*;