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

use clap::{Parser, Subcommand};
use log::{error, info, LevelFilter};
use std::path::PathBuf;
use std::process;

use php2ir::compiler::{Compiler, CompilerOptions};
use php2ir::error::CompileError;

#[derive(Parser)]
#[command(name = "php2ir")]
#[command(about = "PHP 8.x → LLVM-IR → native ELF/EXE/Mach-O compiler")]
#[command(version)]
struct Cli {
    /// Input PHP file
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Emit LLVM IR only
    #[arg(long)]
    emit_llvm: bool,

    /// Emit LLVM IR only (no object file)
    #[arg(long)]
    emit_llvm_only: bool,

    /// Optimization level
    #[arg(long, value_name = "LEVEL", default_value = "O2")]
    opt: String,

    /// LTO mode
    #[arg(long, value_name = "MODE")]
    lto: Option<String>,

    /// PGO generation mode
    #[arg(long)]
    pgo_gen: bool,

    /// PGO use mode with profile data
    #[arg(long, value_name = "PROFDATA")]
    pgo_use: Option<PathBuf>,

    /// Target triple
    #[arg(long, value_name = "TRIPLE")]
    target: Option<String>,

    /// Standard library path
    #[arg(long, value_name = "PATH")]
    stdlib: Option<PathBuf>,

    /// Disable runtime library
    #[arg(long)]
    no_rt: bool,

    /// Sanitizer
    #[arg(long, value_name = "SANITIZER")]
    sanitize: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse PHP file and show AST
    Parse {
        /// Input PHP file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
    },
    /// Show LLVM IR
    Ir {
        /// Input PHP file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
    },
    /// Run tests
    Test {
        /// Test directory
        #[arg(value_name = "DIR")]
        dir: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::Builder::new()
        .filter_level(log_level)
        .init();

    info!("php2ir compiler starting...");

    match cli.command {
        Some(Commands::Parse { input }) => {
            if let Err(e) = parse_php_file(&input) {
                error!("Parse error: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Ir { input }) => {
            if let Err(e) = show_ir(&input) {
                error!("IR generation error: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Test { dir }) => {
            if let Err(e) = run_tests(dir) {
                error!("Test error: {}", e);
                process::exit(1);
            }
        }
        None => {
            // Main compilation path
            if let Err(e) = compile_php(&cli) {
                error!("Compilation error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn compile_php(cli: &Cli) -> Result<(), CompileError> {
    let output = cli.output.clone().unwrap_or_else(|| {
        let mut path = cli.input.clone();
        path.set_extension("");
        path
    });

    let options = CompilerOptions {
        input: cli.input.clone(),
        output,
        emit_llvm: cli.emit_llvm,
        emit_llvm_only: cli.emit_llvm_only,
        optimization_level: cli.opt.clone(),
        lto: cli.lto.clone(),
        pgo_gen: cli.pgo_gen,
        pgo_use: cli.pgo_use.clone(),
        target: cli.target.clone(),
        stdlib: cli.stdlib.clone(),
        no_runtime: cli.no_rt,
        sanitizer: cli.sanitize.clone(),
    };

    info!("Compiling {} to {}", cli.input.display(), output.display());
    
    let mut compiler = Compiler::new(options)?;
    compiler.compile()?;

    info!("Compilation successful!");
    Ok(())
}

fn parse_php_file(input: &PathBuf) -> Result<(), CompileError> {
    info!("Parsing PHP file: {}", input.display());
    
    let options = CompilerOptions {
        input: input.clone(),
        output: PathBuf::from("/dev/null"),
        emit_llvm: false,
        emit_llvm_only: false,
        optimization_level: "O0".to_string(),
        lto: None,
        pgo_gen: false,
        pgo_use: None,
        target: None,
        stdlib: None,
        no_runtime: false,
        sanitizer: None,
    };

    let mut compiler = Compiler::new(options)?;
    let ast = compiler.parse()?;
    
    println!("AST:");
    println!("{:#?}", ast);
    Ok(())
}

fn show_ir(input: &PathBuf) -> Result<(), CompileError> {
    info!("Generating IR for: {}", input.display());
    
    let options = CompilerOptions {
        input: input.clone(),
        output: PathBuf::from("/dev/null"),
        emit_llvm: true,
        emit_llvm_only: true,
        optimization_level: "O0".to_string(),
        lto: None,
        pgo_gen: false,
        pgo_use: None,
        target: None,
        stdlib: None,
        no_runtime: false,
        sanitizer: None,
    };

    let mut compiler = Compiler::new(options)?;
    let ir = compiler.generate_ir()?;
    
    println!("LLVM IR:");
    println!("{}", ir);
    Ok(())
}

fn run_tests(dir: Option<PathBuf>) -> Result<(), CompileError> {
    let test_dir = dir.unwrap_or_else(|| PathBuf::from("tests"));
    info!("Running tests in: {}", test_dir.display());
    
    // TODO: Implement test runner
    info!("Test runner not yet implemented");
    Ok(())
}
