/*
 * Copyright 2025 Mehmet T. AKALIN
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use log::{info, warn, error};

/// File utilities
pub mod file {
    use super::*;
    
    /// Check if file exists and is readable
    pub fn is_readable<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();
        path.exists() && path.is_file() && fs::metadata(path).map(|m| m.permissions().readonly()).unwrap_or(false)
    }
    
    /// Get file extension
    pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
    
    /// Check if file is PHP source
    pub fn is_php_file<P: AsRef<Path>>(path: P) -> bool {
        get_extension(path) == Some("php".to_string())
    }
    
    /// Read file content safely
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }
    
    /// Write file content safely
    pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), std::io::Error> {
        fs::write(path, content)
    }
    
    /// Create directory if it doesn't exist
    pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
        fs::create_dir_all(path)
    }
    
    /// Get file size in bytes
    pub fn get_file_size<P: AsRef<Path>>(path: P) -> Result<u64, std::io::Error> {
        fs::metadata(path).map(|m| m.len())
    }
    
    /// Get file modification time
    pub fn get_modified_time<P: AsRef<Path>>(path: P) -> Result<std::time::SystemTime, std::io::Error> {
        fs::metadata(path).map(|m| m.modified().unwrap_or(m.created().unwrap_or(m.accessed().unwrap())))
    }
}

/// Path utilities
pub mod path {
    use super::*;
    
    /// Normalize path (resolve . and ..)
    pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        let mut components = Vec::new();
        
        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    if components.last().map(|c| c != &std::path::Component::ParentDir).unwrap_or(true) {
                        components.pop();
                    } else {
                        components.push(component);
                    }
                }
                std::path::Component::CurDir => {
                    // Skip current directory
                }
                _ => {
                    components.push(component);
                }
            }
        }
        
        components.into_iter().collect()
    }
    
    /// Get relative path from base
    pub fn relative_to<P: AsRef<Path>, Q: AsRef<Path>>(path: P, base: Q) -> Option<PathBuf> {
        let path = path.as_ref();
        let base = base.as_ref();
        
        if path.starts_with(base) {
            path.strip_prefix(base).ok()
        } else {
            None
        }
    }
    
    /// Check if path is absolute
    pub fn is_absolute<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().is_absolute()
    }
    
    /// Convert path to absolute
    pub fn to_absolute<P: AsRef<Path>>(path: P) -> Result<PathBuf, std::io::Error> {
        let path = path.as_ref();
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            std::env::current_dir().map(|current| current.join(path))
        }
    }
}

/// String utilities
pub mod string {
    use super::*;
    
    /// Escape string for shell
    pub fn shell_escape(s: &str) -> String {
        if s.is_empty() || s.contains(char::is_whitespace) || s.contains('\\') || s.contains('"') || s.contains('\'') {
            format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\""))
        } else {
            s.to_string()
        }
    }
    
    /// Escape string for C
    pub fn c_escape(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\\' => "\\\\".to_string(),
                '"' => "\\\"".to_string(),
                '\'' => "\\\'".to_string(),
                _ if c.is_ascii() && !c.is_control() => c.to_string(),
                _ => format!("\\x{:02x}", c as u8),
            })
            .collect()
    }
    
    /// Escape string for LLVM IR
    pub fn llvm_escape(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '\\' => "\\5C".to_string(),
                '"' => "\\22".to_string(),
                '\n' => "\\0A".to_string(),
                '\r' => "\\0D".to_string(),
                '\t' => "\\09".to_string(),
                _ if c.is_ascii() && !c.is_control() => c.to_string(),
                _ => format!("\\{:02X}", c as u8),
            })
            .collect()
    }
    
    /// Convert string to valid identifier
    pub fn to_identifier(s: &str) -> String {
        let mut result = String::new();
        let mut first = true;
        
        for c in s.chars() {
            if first {
                if c.is_alphabetic() || c == '_' {
                    result.push(c);
                    first = false;
                }
            } else {
                if c.is_alphanumeric() || c == '_' {
                    result.push(c);
                } else {
                    result.push('_');
                }
            }
        }
        
        if result.is_empty() {
            result.push_str("_");
        }
        
        result
    }
    
    /// Convert string to valid C identifier
    pub fn to_c_identifier(s: &str) -> String {
        let mut result = String::new();
        let mut first = true;
        
        for c in s.chars() {
            if first {
                if c.is_alphabetic() || c == '_' {
                    result.push(c);
                    first = false;
                }
            } else {
                if c.is_alphanumeric() || c == '_' {
                    result.push(c);
                } else {
                    result.push('_');
                }
            }
        }
        
        if result.is_empty() {
            result.push_str("_");
        }
        
        result
    }
}

/// Process utilities
pub mod process {
    use super::*;
    use std::process::{Command, Output};
    
    /// Run command and return output
    pub fn run_command(cmd: &str, args: &[&str]) -> Result<Output, std::io::Error> {
        info!("Running command: {} {}", cmd, args.join(" "));
        
        let output = Command::new(cmd)
            .args(args)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Command failed: {}", stderr);
        }
        
        Ok(output)
    }
    
    /// Check if command exists
    pub fn command_exists(cmd: &str) -> bool {
        Command::new(cmd)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// Get command version
    pub fn get_command_version(cmd: &str) -> Option<String> {
        Command::new(cmd)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .map(|s| s.lines().next().unwrap_or("").trim().to_string())
    }
}

/// Hash utilities
pub mod hash {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    /// Calculate hash of string
    pub fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Calculate hash of bytes
    pub fn hash_bytes(bytes: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Calculate hash of file
    pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<u64, std::io::Error> {
        let content = fs::read(path)?;
        Ok(hash_bytes(&content))
    }
}

/// Time utilities
pub mod time {
    use super::*;
    use std::time::{Instant, Duration};
    
    /// Measure execution time of function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Format duration in human readable format
    pub fn format_duration(duration: Duration) -> String {
        if duration.as_secs() > 0 {
            format!("{:.2}s", duration.as_secs_f64())
        } else if duration.as_millis() > 0 {
            format!("{}ms", duration.as_millis())
        } else if duration.as_micros() > 0 {
            format!("{}μs", duration.as_micros())
        } else {
            format!("{}ns", duration.as_nanos())
        }
    }
}

/// Environment utilities
pub mod env {
    use super::*;
    
    /// Get environment variable with default
    pub fn get_env_or_default(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }
    
    /// Check if environment variable is set
    pub fn is_env_set(key: &str) -> bool {
        std::env::var(key).is_ok()
    }
    
    /// Get environment variable as boolean
    pub fn get_env_bool(key: &str, default: bool) -> bool {
        std::env::var(key)
            .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(default)
    }
    
    /// Get environment variable as integer
    pub fn get_env_int(key: &str, default: i64) -> i64 {
        std::env::var(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }
}

/// Configuration utilities
pub mod config {
    use super::*;
    use std::collections::HashMap;
    
    /// Parse key=value configuration string
    pub fn parse_config_string(s: &str) -> HashMap<String, String> {
        let mut config = HashMap::new();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos + 1..].trim();
                
                if !key.is_empty() {
                    config.insert(key.to_string(), value.to_string());
                }
            }
        }
        
        config
    }
    
    /// Merge configuration maps
    pub fn merge_config(mut base: HashMap<String, String>, override_config: HashMap<String, String>) -> HashMap<String, String> {
        for (key, value) in override_config {
            base.insert(key, value);
        }
        base
    }
}

/// Validation utilities
pub mod validation {
    use super::*;
    
    /// Validate file path
    pub fn validate_file_path<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();
        
        if path.to_string_lossy().is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        if path.to_string_lossy().contains('\0') {
            return Err("Path cannot contain null characters".to_string());
        }
        
        Ok(())
    }
    
    /// Validate PHP version string
    pub fn validate_php_version(version: &str) -> Result<(), String> {
        let parts: Vec<&str> = version.split('.').collect();
        
        if parts.len() < 2 {
            return Err("Version must have at least major and minor parts".to_string());
        }
        
        for part in &parts {
            if !part.chars().all(|c| c.is_digit(10)) {
                return Err("Version parts must be numeric".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Validate target triple
    pub fn validate_target_triple(target: &str) -> Result<(), String> {
        let parts: Vec<&str> = target.split('-').collect();
        
        if parts.len() < 3 {
            return Err("Target triple must have at least 3 parts".to_string());
        }
        
        // Basic validation - could be more sophisticated
        if parts[0].is_empty() || parts[1].is_empty() || parts[2].is_empty() {
            return Err("Target triple parts cannot be empty".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_utilities() {
        assert!(file::is_php_file("test.php"));
        assert!(!file::is_php_file("test.txt"));
        assert_eq!(file::get_extension("test.php"), Some("php".to_string()));
    }

    #[test]
    fn test_string_utilities() {
        assert_eq!(string::shell_escape("hello world"), "\"hello world\"");
        assert_eq!(string::to_identifier("hello-world"), "hello_world");
        assert_eq!(string::c_escape("hello\nworld"), "hello\\nworld");
    }

    #[test]
    fn test_path_utilities() {
        assert!(path::is_absolute("/absolute/path"));
        assert!(!path::is_absolute("relative/path"));
    }

    #[test]
    fn test_hash_utilities() {
        let hash1 = hash::hash_string("hello");
        let hash2 = hash::hash_string("hello");
        assert_eq!(hash1, hash2);
        
        let hash3 = hash::hash_string("world");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_time_utilities() {
        let (_, duration) = time::measure_time(|| {
            std::thread::sleep(Duration::from_millis(1));
        });
        assert!(duration.as_millis() >= 1);
    }

    #[test]
    fn test_validation_utilities() {
        assert!(validation::validate_php_version("8.1").is_ok());
        assert!(validation::validate_php_version("invalid").is_err());
        
        assert!(validation::validate_target_triple("x86_64-unknown-linux-gnu").is_ok());
        assert!(validation::validate_target_triple("invalid").is_err());
    }
}
