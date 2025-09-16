use anyhow::Result;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
/// Unix-specific module for detecting Claude Code binary installations
/// Supports macOS, Linux, and other Unix-like systems
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

/// Type of Claude installation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallationType {
    /// System-installed binary
    System,
    /// Custom path specified by user
    Custom,
}

/// Represents a Claude installation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeInstallation {
    /// Full path to the Claude binary
    pub path: String,
    /// Version string if available
    pub version: Option<String>,
    /// Source of discovery (e.g., "nvm", "system", "homebrew", "which")
    pub source: String,
    /// Type of installation
    pub installation_type: InstallationType,
}

/// Main function to find the Claude binary on Unix systems
/// Checks database first for stored path and preference, then prioritizes accordingly
pub fn find_claude_binary(app_handle: &tauri::AppHandle) -> Result<String, String> {
    info!("Searching for claude binary on Unix system...");

    // First check if we have a stored path and preference in the database
    if let Ok(app_data_dir) = app_handle.path().app_data_dir() {
        let db_path = app_data_dir.join("agents.db");
        if db_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                // Check for stored path first
                if let Ok(stored_path) = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'claude_binary_path'",
                    [],
                    |row| row.get::<_, String>(0),
                ) {
                    info!("Found stored claude path in database: {}", stored_path);
                    
                    // Check if the path still exists
                    let path_buf = PathBuf::from(&stored_path);
                    if path_buf.exists() && path_buf.is_file() {
                        // Test if the binary is actually executable
                        if test_claude_binary(&stored_path) {
                            info!("Using cached Claude CLI path: {}", stored_path);
                            return Ok(stored_path);
                        } else {
                            warn!("Stored claude path exists but is not executable: {}", stored_path);
                            // Remove invalid cached path
                            let _ = conn.execute(
                                "DELETE FROM app_settings WHERE key = 'claude_binary_path'",
                                [],
                            );
                        }
                    } else {
                        warn!("Stored claude path no longer exists: {}", stored_path);
                        // Remove invalid cached path
                        let _ = conn.execute(
                            "DELETE FROM app_settings WHERE key = 'claude_binary_path'",
                            [],
                        );
                    }
                }
                
                // Check user preference
                let preference = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'claude_installation_preference'",
                    [],
                    |row| row.get::<_, String>(0),
                ).unwrap_or_else(|_| "system".to_string());
                
                info!("User preference for Claude installation: {}", preference);
            }
        }
    }

    // Discover all available system installations
    let installations = discover_unix_installations();

    if installations.is_empty() {
        error!("Could not find claude binary in any Unix location");
        return Err("Claude Code not found. Please ensure it's installed in one of these locations: PATH, /usr/local/bin, /opt/homebrew/bin, ~/.nvm/versions/node/*/bin, ~/.claude/local, ~/.local/bin".to_string());
    }

    // Log all found installations
    for installation in &installations {
        info!("Found Claude installation: {:?}", installation);
    }

    // Select the best installation (highest version)
    if let Some(best) = select_best_installation(installations) {
        info!(
            "Selected Claude installation: path={}, version={:?}, source={}",
            best.path, best.version, best.source
        );
        
        // Store the successful path in database for future use
        if let Err(e) = store_claude_path(app_handle, &best.path) {
            warn!("Failed to store claude path in database: {}", e);
        }
        
        Ok(best.path)
    } else {
        Err("No valid Claude installation found".to_string())
    }
}

/// Store Claude CLI path in database for future use
fn store_claude_path(app_handle: &tauri::AppHandle, path: &str) -> Result<(), String> {
    if let Ok(app_data_dir) = app_handle.path().app_data_dir() {
        if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
            return Err(format!("Failed to create app data directory: {}", e));
        }
        
        let db_path = app_data_dir.join("agents.db");
        match rusqlite::Connection::open(&db_path) {
            Ok(conn) => {
                // Create table if it doesn't exist
                if let Err(e) = conn.execute(
                    "CREATE TABLE IF NOT EXISTS app_settings (
                        key TEXT PRIMARY KEY,
                        value TEXT NOT NULL
                    )",
                    [],
                ) {
                    return Err(format!("Failed to create settings table: {}", e));
                }
                
                // Store the path
                if let Err(e) = conn.execute(
                    "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
                    rusqlite::params!["claude_binary_path", path],
                ) {
                    return Err(format!("Failed to store claude path: {}", e));
                }
                
                info!("Stored claude path in database: {}", path);
                Ok(())
            }
            Err(e) => Err(format!("Failed to open database: {}", e)),
        }
    } else {
        Err("Failed to get app data directory".to_string())
    }
}

/// Test if a Claude binary is actually functional on Unix
fn test_claude_binary(path: &str) -> bool {
    debug!("Testing Claude binary at: {}", path);

    // Test with a simple --version command
    let mut cmd = create_command_with_env(path);
    cmd.arg("--version");
    
    match cmd.output() {
        Ok(output) => {
            let success = output.status.success();
            debug!("Claude binary test result: success={}", success);
            success
        }
        Err(e) => {
            debug!("Failed to test Claude binary: {}", e);
            false
        }
    }
}

/// Discovers all available Claude installations and returns them for selection on Unix
/// This allows UI to show a version selector
pub fn discover_claude_installations() -> Vec<ClaudeInstallation> {
    info!("Discovering all Unix Claude installations...");

    let mut installations = discover_unix_installations();

    // Sort by version (highest first), then by source preference
    installations.sort_by(|a, b| {
        match (&a.version, &b.version) {
            (Some(v1), Some(v2)) => {
                // Compare versions in descending order (newest first)
                match compare_versions(v2, v1) {
                    Ordering::Equal => {
                        // If versions are equal, prefer by source
                        source_preference(a).cmp(&source_preference(b))
                    }
                    other => other,
                }
            }
            (Some(_), None) => Ordering::Less, // Version comes before no version
            (None, Some(_)) => Ordering::Greater,
            (None, None) => source_preference(a).cmp(&source_preference(b)),
        }
    });

    installations
}

/// Returns a preference score for installation sources (lower is better)
fn source_preference(installation: &ClaudeInstallation) -> u8 {
    match installation.source.as_str() {
        "which" => 1,
        "homebrew-arm" | "homebrew-intel" => 2,
        "homebrew" => 3,
        "usr-local" => 4,
        "system" => 5,
        source if source.starts_with("nvm") => 6,
        "local-bin" => 7,
        "claude-local" => 8,
        "npm-global" => 9,
        "yarn" | "yarn-global" => 10,
        "bun" => 11,
        "pnpm" => 12,
        "node-modules" => 13,
        "home-bin" => 14,
        "snap" => 15,
        "flatpak-system" => 16,
        "flatpak-user" => 17,
        "appimage-user" => 18,
        "macports" => 19,
        "PATH" => 20,
        _ => 21,
    }
}

/// Discovers all Claude installations on Unix systems (macOS, Linux, etc.)
fn discover_unix_installations() -> Vec<ClaudeInstallation> {
    let mut installations = Vec::new();

    // 1. Try 'which' command first (now works in production)
    if let Some(installation) = try_which_command() {
        installations.push(installation);
    }

    // 2. Check NVM paths
    installations.extend(find_nvm_installations());

    // 3. Check standard paths
    installations.extend(find_standard_installations());

    // 4. Check Unix-specific paths
    installations.extend(find_unix_specific_installations());

    // Remove duplicates by path
    let mut unique_paths = std::collections::HashSet::new();
    installations.retain(|install| unique_paths.insert(install.path.clone()));

    // Test each installation for actual functionality
    installations.retain(|install| {
        let is_functional = test_claude_binary(&install.path);
        if !is_functional {
            warn!("Claude installation at {} is not functional, removing from list", install.path);
        }
        is_functional
    });

    installations
}

/// Try using the 'which' command to find Claude
fn try_which_command() -> Option<ClaudeInstallation> {
    debug!("Trying 'which claude' to find binary...");

    let mut cmd = create_command_with_env("which");
    cmd.arg("claude");
    match cmd.output() {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if output_str.is_empty() {
                return None;
            }

            // Parse aliased output: "claude: aliased to /path/to/claude"
            let path = if output_str.starts_with("claude:") && output_str.contains("aliased to") {
                output_str
                    .split("aliased to")
                    .nth(1)
                    .map(|s| s.trim().to_string())
            } else {
                Some(output_str)
            }?;

            debug!("'which' found claude at: {}", path);

            // Verify the path exists
            if !PathBuf::from(&path).exists() {
                warn!("Path from 'which' does not exist: {}", path);
                return None;
            }

            // Get version
            let version = get_claude_version(&path).ok().flatten();

            Some(ClaudeInstallation {
                path,
                version,
                source: "which".to_string(),
                installation_type: InstallationType::System,
            })
        }
        _ => None,
    }
}

/// Find Claude installations in NVM directories
fn find_nvm_installations() -> Vec<ClaudeInstallation> {
    let mut installations = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let nvm_dir = PathBuf::from(&home)
            .join(".nvm")
            .join("versions")
            .join("node");

        debug!("Checking NVM directory: {:?}", nvm_dir);

        if let Ok(entries) = std::fs::read_dir(&nvm_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let claude_path = entry.path().join("bin").join("claude");

                    if claude_path.exists() && claude_path.is_file() {
                        let path_str = claude_path.to_string_lossy().to_string();
                        let node_version = entry.file_name().to_string_lossy().to_string();

                        debug!("Found Claude in NVM node {}: {}", node_version, path_str);

                        // Get Claude version
                        let version = get_claude_version(&path_str).ok().flatten();

                        installations.push(ClaudeInstallation {
                            path: path_str,
                            version,
                            source: format!("nvm ({})", node_version),
                            installation_type: InstallationType::System,
                        });
                    }
                }
            }
        }
    }

    installations
}

/// Check standard installation paths for Unix systems
fn find_standard_installations() -> Vec<ClaudeInstallation> {
    let mut installations = Vec::new();

    // Common installation paths for claude on Unix systems
    let mut paths_to_check: Vec<(String, String)> = vec![
        // Homebrew paths (macOS)
        ("/usr/local/bin/claude".to_string(), "homebrew-intel".to_string()),
        ("/opt/homebrew/bin/claude".to_string(), "homebrew-arm".to_string()),
        // System-wide paths (Linux and macOS)
        ("/usr/local/bin/claude".to_string(), "usr-local".to_string()),
        ("/usr/bin/claude".to_string(), "usr-bin".to_string()),
        ("/bin/claude".to_string(), "bin".to_string()),
    ];

    // Also check user-specific paths
    if let Ok(home) = std::env::var("HOME") {
        paths_to_check.extend(vec![
            (
                format!("{}/.claude/local/claude", home),
                "claude-local".to_string(),
            ),
            (
                format!("{}/.local/bin/claude", home),
                "local-bin".to_string(),
            ),
            (
                format!("{}/bin/claude", home),
                "home-bin".to_string(),
            ),
            (
                format!("{}/.npm-global/bin/claude", home),
                "npm-global".to_string(),
            ),
            (format!("{}/.yarn/bin/claude", home), "yarn".to_string()),
            (format!("{}/.bun/bin/claude", home), "bun".to_string()),
            (
                format!("{}/.pnpm/claude", home),
                "pnpm".to_string(),
            ),
            // Check common node_modules locations
            (
                format!("{}/node_modules/.bin/claude", home),
                "node-modules".to_string(),
            ),
            (
                format!("{}/.config/yarn/global/node_modules/.bin/claude", home),
                "yarn-global".to_string(),
            ),
        ]);
    }

    // Check each path
    for (path, source) in paths_to_check {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found claude at standard path: {} ({})", path, source);

            // Get version
            let version = get_claude_version(&path).ok().flatten();

            installations.push(ClaudeInstallation {
                path,
                version,
                source,
                installation_type: InstallationType::System,
            });
        }
    }

    // Also check if claude is available in PATH (without full path)
    let mut cmd = create_command_with_env("claude");
    cmd.arg("--version");
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            debug!("claude is available in PATH");
            let version = extract_version_from_output(&output.stdout);

            installations.push(ClaudeInstallation {
                path: "claude".to_string(),
                version,
                source: "PATH".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    installations
}

/// Find Unix-specific Claude installations (macOS, Linux specific paths)
fn find_unix_specific_installations() -> Vec<ClaudeInstallation> {
    let mut installations = Vec::new();

    // Check Homebrew Cellar for different versions (macOS)
    let homebrew_paths = [
        "/usr/local/Cellar", // Intel Mac
        "/opt/homebrew/Cellar", // Apple Silicon Mac
    ];

    for cellar_path in &homebrew_paths {
        let claude_cellar = PathBuf::from(cellar_path).join("claude");
        if claude_cellar.exists() {
            if let Ok(entries) = std::fs::read_dir(&claude_cellar) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        let claude_bin = entry.path().join("bin").join("claude");
                        if claude_bin.exists() && claude_bin.is_file() {
                            let path_str = claude_bin.to_string_lossy().to_string();
                            let version_str = entry.file_name().to_string_lossy().to_string();
                            
                            debug!("Found Claude in Homebrew Cellar: {}", path_str);
                            
                            let version = get_claude_version(&path_str).ok().flatten();
                            let source = if cellar_path.contains("opt/homebrew") {
                                format!("homebrew-arm ({})", version_str)
                            } else {
                                format!("homebrew-intel ({})", version_str)
                            };
                            
                            installations.push(ClaudeInstallation {
                                path: path_str,
                                version,
                                source,
                                installation_type: InstallationType::System,
                            });
                        }
                    }
                }
            }
        }
    }

    // Check for package manager installations by looking for desktop files (Linux)
    let desktop_file_paths = vec![
        "/usr/share/applications/claude.desktop",
        "/usr/local/share/applications/claude.desktop",
        "/var/lib/snapd/desktop/applications/claude*.desktop",
        "/var/lib/flatpak/exports/share/applications/claude*.desktop",
    ];

    for path_pattern in desktop_file_paths {
        if path_pattern.contains('*') {
            // Handle glob patterns for Snap and Flatpak
            if let Ok(paths) = glob::glob(path_pattern) {
                for path in paths.flatten() {
                    if let Some(installation) = extract_binary_from_desktop_file(&path, "package-manager") {
                        installations.push(installation);
                    }
                }
            }
        } else {
            // Handle direct paths
            let path = PathBuf::from(path_pattern);
            if path.exists() {
                if let Some(installation) = extract_binary_from_desktop_file(&path, "system-package") {
                    installations.push(installation);
                }
            }
        }
    }

    // Check Snap installations directly (Linux)
    let snap_paths = vec![
        "/snap/bin/claude",
        "/var/lib/snapd/snap/bin/claude",
    ];

    for snap_path in snap_paths {
        let path_buf = PathBuf::from(snap_path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found Claude Snap installation: {}", snap_path);
            let version = get_claude_version(snap_path).ok().flatten();
            installations.push(ClaudeInstallation {
                path: snap_path.to_string(),
                version,
                source: "snap".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    // Check Flatpak installations (Linux)
    let flatpak_paths = vec![
        "/var/lib/flatpak/exports/bin/claude",
        "/usr/local/share/flatpak/exports/bin/claude",
    ];

    // Also check user-specific Flatpak
    if let Ok(home) = std::env::var("HOME") {
        let user_flatpak = format!("{}/.local/share/flatpak/exports/bin/claude", home);
        if PathBuf::from(&user_flatpak).exists() {
            debug!("Found Claude Flatpak user installation: {}", user_flatpak);
            let version = get_claude_version(&user_flatpak).ok().flatten();
            installations.push(ClaudeInstallation {
                path: user_flatpak,
                version,
                source: "flatpak-user".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    for flatpak_path in flatpak_paths {
        let path_buf = PathBuf::from(flatpak_path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found Claude Flatpak installation: {}", flatpak_path);
            let version = get_claude_version(flatpak_path).ok().flatten();
            installations.push(ClaudeInstallation {
                path: flatpak_path.to_string(),
                version,
                source: "flatpak-system".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    // Check AppImage installations (Linux)
    if let Ok(home) = std::env::var("HOME") {
        let appimage_locations = vec![
            format!("{}/Applications", home),
            format!("{}/.local/bin", home),
            format!("{}/bin", home),
            format!("{}/Downloads", home),
        ];

        for location in appimage_locations {
            if let Ok(entries) = std::fs::read_dir(&location) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_lowercase();
                    if (file_name.starts_with("claude") || file_name.contains("claude")) 
                        && file_name.ends_with(".appimage") {
                        let path_str = entry.path().to_string_lossy().to_string();
                        debug!("Found Claude AppImage: {}", path_str);
                        
                        let version = get_claude_version(&path_str).ok().flatten();
                        installations.push(ClaudeInstallation {
                            path: path_str,
                            version,
                            source: "appimage-user".to_string(),
                            installation_type: InstallationType::System,
                        });
                    }
                }
            }
        }
    }

    // Additional Unix-specific locations (both macOS and Linux)
    let unix_specific_paths = vec![
        // macOS Application bundle paths
        ("/Applications/Claude.app/Contents/MacOS/claude".to_string(), "app-bundle".to_string()),
        ("/Applications/Claude CLI.app/Contents/MacOS/claude".to_string(), "app-bundle".to_string()),
        // Terminal app installations (macOS)
        ("/usr/local/opt/claude/bin/claude".to_string(), "homebrew-formula".to_string()),
        ("/opt/homebrew/opt/claude/bin/claude".to_string(), "homebrew-formula".to_string()),
        // MacPorts (macOS)
        ("/opt/local/bin/claude".to_string(), "macports".to_string()),
        ("/opt/local/share/claude/bin/claude".to_string(), "macports-share".to_string()),
        // Developer tools locations (macOS)
        ("/Developer/usr/bin/claude".to_string(), "developer-tools".to_string()),
        // Xcode command line tools (macOS)
        ("/Library/Developer/CommandLineTools/usr/bin/claude".to_string(), "xcode-cli".to_string()),
        // Linux-specific system paths
        ("/opt/claude/bin/claude".to_string(), "opt-claude".to_string()),
        ("/opt/claude/claude".to_string(), "opt-claude-direct".to_string()),
        ("/usr/lib/claude/claude".to_string(), "usr-lib".to_string()),
        ("/usr/libexec/claude".to_string(), "usr-libexec".to_string()),
        // Distribution-specific paths (Linux)
        ("/usr/share/claude/bin/claude".to_string(), "usr-share".to_string()),
    ];

    // Check each Unix-specific path
    for (path, source) in unix_specific_paths {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found Claude at Unix-specific path: {} ({})", path, source);

            let version = get_claude_version(&path).ok().flatten();

            installations.push(ClaudeInstallation {
                path,
                version,
                source,
                installation_type: InstallationType::System,
            });
        }
    }

    installations
}

/// Extract binary path from .desktop file
fn extract_binary_from_desktop_file(desktop_path: &PathBuf, source_prefix: &str) -> Option<ClaudeInstallation> {
    if let Ok(content) = std::fs::read_to_string(desktop_path) {
        for line in content.lines() {
            if line.starts_with("Exec=") {
                if let Some(exec_path) = line.split('=').nth(1) {
                    // Extract the binary path (first word before any arguments)
                    let binary_path = exec_path.split_whitespace().next().unwrap_or("");
                    if !binary_path.is_empty() && PathBuf::from(binary_path).exists() {
                        debug!("Found Claude binary from desktop file: {}", binary_path);
                        let version = get_claude_version(binary_path).ok().flatten();
                        let source = if desktop_path.to_string_lossy().contains("snap") {
                            "snap-desktop".to_string()
                        } else if desktop_path.to_string_lossy().contains("flatpak") {
                            "flatpak-desktop".to_string()
                        } else {
                            source_prefix.to_string()
                        };
                        
                        return Some(ClaudeInstallation {
                            path: binary_path.to_string(),
                            version,
                            source,
                            installation_type: InstallationType::System,
                        });
                    }
                }
                break;
            }
        }
    }
    None
}

/// Get Claude version by running --version command on Unix
fn get_claude_version(path: &str) -> Result<Option<String>, String> {
    let mut cmd = create_command_with_env(path);
    cmd.arg("--version");
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(extract_version_from_output(&output.stdout))
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            warn!("Failed to get version for {}: {}", path, e);
            Ok(None)
        }
    }
}

/// Extract version string from command output
fn extract_version_from_output(stdout: &[u8]) -> Option<String> {
    let output_str = String::from_utf8_lossy(stdout);
    
    // Debug log the raw output
    debug!("Raw version output: {:?}", output_str);
    
    // Use regex to directly extract version pattern (e.g., "1.0.41")
    // This pattern matches:
    // - One or more digits, followed by
    // - A dot, followed by
    // - One or more digits, followed by
    // - A dot, followed by
    // - One or more digits
    // - Optionally followed by pre-release/build metadata
    let version_regex = regex::Regex::new(r"(\d+\.\d+\.\d+(?:-[a-zA-Z0-9.-]+)?(?:\+[a-zA-Z0-9.-]+)?)").ok()?;
    
    if let Some(captures) = version_regex.captures(&output_str) {
        if let Some(version_match) = captures.get(1) {
            let version = version_match.as_str().to_string();
            debug!("Extracted version: {:?}", version);
            return Some(version);
        }
    }
    
    debug!("No version found in output");
    None
}

/// Select the best installation based on version and source preference
fn select_best_installation(installations: Vec<ClaudeInstallation>) -> Option<ClaudeInstallation> {
    // In production builds, version information may not be retrievable because
    // spawning external processes can be restricted. We therefore no longer
    // discard installations that lack a detected version – the mere presence
    // of a readable binary on disk is enough to consider it valid. We still
    // prefer binaries with version information when it is available so that
    // in development builds we keep the previous behaviour of picking the
    // most recent version.
    installations.into_iter().max_by(|a, b| {
        match (&a.version, &b.version) {
            // If both have versions, compare them semantically first, then by source preference
            (Some(v1), Some(v2)) => {
                match compare_versions(v1, v2) {
                    Ordering::Equal => {
                        // If versions are equal, prefer by source (lower score is better)
                        source_preference(b).cmp(&source_preference(a))
                    }
                    other => other,
                }
            }
            // Prefer the entry that actually has version information.
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            // Neither have version info: prefer by source, then avoid bare "claude" paths
            (None, None) => {
                match source_preference(a).cmp(&source_preference(b)) {
                    Ordering::Equal => {
                        if a.path == "claude" && b.path != "claude" {
                            Ordering::Less
                        } else if a.path != "claude" && b.path == "claude" {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                    other => other.reverse(), // Reverse because lower source_preference score is better
                }
            }
        }
    })
}

/// Compare two version strings
fn compare_versions(a: &str, b: &str) -> Ordering {
    // Simple semantic version comparison
    let a_parts: Vec<u32> = a
        .split('.')
        .filter_map(|s| {
            // Handle versions like "1.0.17-beta" by taking only numeric part
            s.chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .ok()
        })
        .collect();

    let b_parts: Vec<u32> = b
        .split('.')
        .filter_map(|s| {
            s.chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .ok()
        })
        .collect();

    // Compare each part
    for i in 0..std::cmp::max(a_parts.len(), b_parts.len()) {
        let a_val = a_parts.get(i).unwrap_or(&0);
        let b_val = b_parts.get(i).unwrap_or(&0);
        match a_val.cmp(b_val) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

/// Build enhanced PATH for macOS applications to find Node.js, Claude CLI, and other tools
/// This is crucial for DMG applications that don't inherit shell environment
fn build_enhanced_path() -> String {
    let mut path_components = Vec::new();

    // 1. Start with current PATH if available
    if let Ok(current_path) = std::env::var("PATH") {
        path_components.push(current_path);
    }

    // 2. Add common development tool paths
    let common_paths = vec![
        // Node.js via NVM (scan all versions, prioritize latest)
        get_latest_nvm_path(),
        // Homebrew paths
        "/opt/homebrew/bin".to_string(),
        "/opt/homebrew/sbin".to_string(),
        "/usr/local/bin".to_string(),
        "/usr/local/sbin".to_string(),
        // System paths
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
        // User paths
        format!("{}/.local/bin", std::env::var("HOME").unwrap_or_default()),
        format!("{}/.cargo/bin", std::env::var("HOME").unwrap_or_default()),
        format!("{}/.bun/bin", std::env::var("HOME").unwrap_or_default()),
    ];

    // Add non-empty paths
    for path in common_paths {
        if !path.is_empty() && std::path::Path::new(&path).exists() {
            path_components.push(path);
        }
    }

    let final_path = path_components.join(":");
    info!("Enhanced PATH for macOS app: {}", final_path);
    final_path
}

/// Get the latest NVM Node.js version path
fn get_latest_nvm_path() -> String {
    if let Ok(home) = std::env::var("HOME") {
        let nvm_versions = std::path::PathBuf::from(&home)
            .join(".nvm")
            .join("versions")
            .join("node");

        if nvm_versions.exists() {
            if let Ok(entries) = std::fs::read_dir(&nvm_versions) {
                let mut versions: Vec<_> = entries
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            if name.starts_with('v') {
                                Some((name, e.path().join("bin")))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                // Sort versions to get the latest
                versions.sort_by(|a, b| {
                    // Simple version comparison (v22.18.0 > v20.0.0)
                    let a_version = &a.0[1..]; // Remove 'v' prefix
                    let b_version = &b.0[1..];
                    compare_versions(b_version, a_version) // Reverse for descending order
                });

                if let Some((_, bin_path)) = versions.first() {
                    let path_str = bin_path.to_string_lossy().to_string();
                    info!("Found latest NVM Node.js path: {}", path_str);
                    return path_str;
                }
            }
        }
    }

    String::new()
}

/// Helper function to create a Command with proper environment variables for Unix
/// This ensures commands like Claude can find Node.js and other dependencies
pub fn create_command_with_env(program: &str) -> Command {
    let mut cmd = Command::new(program);

    info!("Creating command for: {}", program);

    // Build a comprehensive PATH for macOS applications
    let enhanced_path = build_enhanced_path();
    cmd.env("PATH", enhanced_path);

    // Inherit essential environment variables from parent process
    for (key, value) in std::env::vars() {
        // Pass through other essential environment variables (excluding PATH since we build our own)
        if key == "HOME"
            || key == "USER"
            || key == "SHELL"
            || key == "LANG"
            || key == "LC_ALL"
            || key.starts_with("LC_")
            || key == "NODE_PATH"
            || key == "NVM_DIR"
            || key == "NVM_BIN"
            || key == "HOMEBREW_PREFIX"
            || key == "HOMEBREW_CELLAR"
            // Add proxy environment variables (only uppercase)
            || key == "HTTP_PROXY"
            || key == "HTTPS_PROXY"
            || key == "NO_PROXY"
            || key == "ALL_PROXY"
        {
            debug!("Inheriting env var: {}={}", key, value);
            cmd.env(&key, &value);
        }
    }
    
    // Log proxy-related environment variables for debugging
    info!("Command will use proxy settings:");
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY") {
        info!("  HTTP_PROXY={}", http_proxy);
    }
    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY") {
        info!("  HTTPS_PROXY={}", https_proxy);
    }

    // Add NVM support if the program is in an NVM directory
    if program.contains("/.nvm/versions/node/") {
        if let Some(node_bin_dir) = std::path::Path::new(program).parent() {
            // Ensure the Node.js bin directory is in PATH
            let current_path = std::env::var("PATH").unwrap_or_default();
            let node_bin_str = node_bin_dir.to_string_lossy();
            if !current_path.contains(&node_bin_str.as_ref()) {
                let new_path = format!("{}:{}", node_bin_str, current_path);
                debug!("Adding NVM bin directory to PATH: {}", node_bin_str);
                cmd.env("PATH", new_path);
            }
        }
    }

    cmd
}