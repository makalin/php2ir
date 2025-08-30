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

//! php2ir - PHP 8.x → LLVM-IR → native ELF/EXE/Mach-O compiler
//! 
//! This library provides a direct AOT compilation pipeline from PHP source code
//! to native binaries, skipping C as an intermediate step.

pub mod ast;
pub mod compiler;
pub mod error;
pub mod ir;
pub mod parser;
pub mod runtime;
pub mod types;
pub mod utils;

// Re-export main types for convenience
pub use compiler::{Compiler, CompilerOptions};
pub use error::CompileError;
pub use parser::Parser;
pub use types::{Type, Value};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Supported PHP version
pub const PHP_VERSION: &str = "8.0+";

/// Default optimization level
pub const DEFAULT_OPT_LEVEL: &str = "O2";

/// Default target triple
pub const DEFAULT_TARGET: &str = "native";

/// Runtime library name
pub const RUNTIME_LIB: &str = "libphp2ir";

/// Compiler information
pub fn get_compiler_info() -> String {
    format!(
        "php2ir v{} - PHP {} → LLVM IR → Native",
        VERSION, PHP_VERSION
    )
}

/// Check if target is supported
pub fn is_target_supported(target: &str) -> bool {
    let supported = [
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin", 
        "x86_64-pc-windows-gnu",
        "aarch64-unknown-linux-gnu",
        "aarch64-apple-darwin",
        "native",
    ];
    supported.contains(&target)
}

/// Get default optimization flags for a given level
pub fn get_opt_flags(level: &str) -> Vec<&'static str> {
    match level {
        "O0" => vec!["-O0"],
        "O1" => vec!["-O1"],
        "O2" => vec!["-O2"],
        "O3" => vec!["-O3"],
        "Oz" => vec!["-Oz"],
        _ => vec!["-O2"], // Default to O2
    }
}

/// Get LTO flags
pub fn get_lto_flags(lto: &str) -> Vec<&'static str> {
    match lto {
        "thin" => vec!["-flto=thin"],
        "full" => vec!["-flto=full"],
        _ => vec![],
    }
}

/// Get sanitizer flags
pub fn get_sanitizer_flags(sanitizer: &str) -> Vec<&'static str> {
    match sanitizer {
        "address" => vec!["-fsanitize=address"],
        "ubsan" => vec!["-fsanitize=undefined"],
        "thread" => vec!["-fsanitize=thread"],
        "memory" => vec!["-fsanitize=memory"],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_info() {
        let info = get_compiler_info();
        assert!(info.contains("php2ir"));
        assert!(info.contains("PHP"));
        assert!(info.contains("LLVM IR"));
    }

    #[test]
    fn test_target_support() {
        assert!(is_target_supported("x86_64-unknown-linux-gnu"));
        assert!(is_target_supported("native"));
        assert!(!is_target_supported("unsupported-target"));
    }

    #[test]
    fn test_opt_flags() {
        assert_eq!(get_opt_flags("O2"), vec!["-O2"]);
        assert_eq!(get_opt_flags("O0"), vec!["-O0"]);
        assert_eq!(get_opt_flags("invalid"), vec!["-O2"]); // Default
    }

    #[test]
    fn test_lto_flags() {
        assert_eq!(get_lto_flags("thin"), vec!["-flto=thin"]);
        assert_eq!(get_lto_flags("full"), vec!["-flto=full"]);
        assert_eq!(get_lto_flags("invalid"), vec![]);
    }

    #[test]
    fn test_sanitizer_flags() {
        assert_eq!(get_sanitizer_flags("address"), vec!["-fsanitize=address"]);
        assert_eq!(get_sanitizer_flags("ubsan"), vec!["-fsanitize=undefined"]);
        assert_eq!(get_sanitizer_flags("invalid"), vec![]);
    }
}
