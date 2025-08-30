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

use std::fmt;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the php2ir compiler
#[derive(Error, Debug)]
pub enum CompileError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error
    #[error("Parse error in {}: {message}", .file.as_ref().map(|f| f.display()).unwrap_or_else(|| "unknown file".into()))]
    Parse {
        file: Option<PathBuf>,
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },

    /// Type error
    #[error("Type error: {message}")]
    Type {
        message: String,
        location: Option<Location>,
    },

    /// LLVM IR generation error
    #[error("LLVM IR generation error: {0}")]
    IrGeneration(String),

    /// LLVM compilation error
    #[error("LLVM compilation error: {0}")]
    LlvmCompilation(String),

    /// Linking error
    #[error("Linking error: {0}")]
    Linking(String),

    /// Runtime error
    #[error("Runtime error: {0}")]
    Runtime(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Unsupported feature
    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    /// Internal compiler error
    #[error("Internal compiler error: {0}")]
    Internal(String),
}

/// Source location information
#[derive(Debug, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(file: PathBuf, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Result type for compilation operations
pub type CompileResult<T> = Result<T, CompileError>;

/// Error context for better error reporting
pub trait ErrorContext<T> {
    fn with_context<F>(self, f: F) -> Self
    where
        F: FnOnce() -> String;
}

impl<T> ErrorContext<T> for CompileResult<T> {
    fn with_context<F>(self, f: F) -> Self
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| match e {
            CompileError::Parse { file, message, line, column } => {
                CompileError::Parse {
                    file,
                    message: format!("{}: {}", f(), message),
                    line,
                    column,
                }
            }
            CompileError::Type { message, location } => {
                CompileError::Type {
                    message: format!("{}: {}", f(), message),
                    location,
                }
            }
            _ => e,
        })
    }
}

/// Helper macro for creating parse errors
#[macro_export]
macro_rules! parse_error {
    ($file:expr, $message:expr) => {
        CompileError::Parse {
            file: Some($file.into()),
            message: $message.to_string(),
            line: None,
            column: None,
        }
    };
    ($file:expr, $message:expr, $line:expr, $column:expr) => {
        CompileError::Parse {
            file: Some($file.into()),
            message: $message.to_string(),
            line: Some($line),
            column: Some($column),
        }
    };
}

/// Helper macro for creating type errors
#[macro_export]
macro_rules! type_error {
    ($message:expr) => {
        CompileError::Type {
            message: $message.to_string(),
            location: None,
        }
    };
    ($message:expr, $location:expr) => {
        CompileError::Type {
            message: $message.to_string(),
            location: Some($location),
        }
    };
}

/// Helper macro for creating unsupported feature errors
#[macro_export]
macro_rules! unsupported {
    ($feature:expr) => {
        CompileError::Unsupported($feature.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_location_display() {
        let location = Location::new(PathBuf::from("test.php"), 10, 5);
        assert_eq!(location.to_string(), "test.php:10:5");
    }

    #[test]
    fn test_parse_error() {
        let error = parse_error!(Path::new("test.php"), "syntax error");
        match error {
            CompileError::Parse { file, message, line, column } => {
                assert_eq!(file.unwrap().to_string_lossy(), "test.php");
                assert_eq!(message, "syntax error");
                assert_eq!(line, None);
                assert_eq!(column, None);
            }
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn test_type_error() {
        let error = type_error!("type mismatch");
        match error {
            CompileError::Type { message, location } => {
                assert_eq!(message, "type mismatch");
                assert_eq!(location, None);
            }
            _ => panic!("Expected Type error"),
        }
    }

    #[test]
    fn test_unsupported() {
        let error = unsupported!("generators");
        match error {
            CompileError::Unsupported(feature) => {
                assert_eq!(feature, "generators");
            }
            _ => panic!("Expected Unsupported error"),
        }
    }
}
