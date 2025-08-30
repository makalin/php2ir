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

use std::path::PathBuf;
use std::process::Command;
use log::{info, warn, error};
use crate::ast::AstNode;
use crate::error::{CompileError, CompileResult};
use crate::parser::{Parser, DefaultParser};
use crate::types::TypeContext;
use crate::ir::IrGenerator;

/// Compiler options
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Input PHP file
    pub input: PathBuf,
    
    /// Output file path
    pub output: PathBuf,
    
    /// Whether to emit LLVM IR
    pub emit_llvm: bool,
    
    /// Whether to emit LLVM IR only (no object file)
    pub emit_llvm_only: bool,
    
    /// Optimization level
    pub optimization_level: String,
    
    /// LTO mode
    pub lto: Option<String>,
    
    /// PGO generation mode
    pub pgo_gen: bool,
    
    /// PGO use mode with profile data
    pub pgo_use: Option<PathBuf>,
    
    /// Target triple
    pub target: Option<String>,
    
    /// Standard library path
    pub stdlib: Option<PathBuf>,
    
    /// Disable runtime library
    pub no_runtime: bool,
    
    /// Sanitizer
    pub sanitizer: Option<String>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            input: PathBuf::from("input.php"),
            output: PathBuf::from("output"),
            emit_llvm: false,
            emit_llvm_only: false,
            optimization_level: "O2".to_string(),
            lto: None,
            pgo_gen: false,
            pgo_use: None,
            target: None,
            stdlib: None,
            no_runtime: false,
            sanitizer: None,
        }
    }
}

/// Main compiler struct
pub struct Compiler {
    options: CompilerOptions,
    parser: DefaultParser,
    type_context: TypeContext,
    ir_generator: IrGenerator,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new(options: CompilerOptions) -> CompileResult<Self> {
        let parser = DefaultParser::new();
        let type_context = TypeContext::new();
        let ir_generator = IrGenerator::new()?;
        
        Ok(Self {
            options,
            parser,
            type_context,
            ir_generator,
        })
    }
    
    /// Run the full compilation pipeline
    pub fn compile(&mut self) -> CompileResult<()> {
        info!("Starting compilation of {}", self.options.input.display());
        
        // 1. Parse PHP source
        let ast = self.parse()?;
        info!("Parsing completed, {} AST nodes generated", ast.len());
        
        // 2. Type checking and semantic analysis
        self.type_check(&ast)?;
        info!("Type checking completed");
        
        // 3. Generate LLVM IR
        let ir = self.generate_ir()?;
        info!("LLVM IR generation completed");
        
        // 4. Optimize IR
        if self.options.optimization_level != "O0" {
            self.optimize_ir(&ir)?;
            info!("IR optimization completed");
        }
        
        // 5. Generate object file or final binary
        if self.options.emit_llvm_only {
            self.write_ir_file(&ir)?;
            info!("LLVM IR written to {}", self.options.output.display());
        } else {
            self.generate_object_file(&ir)?;
            if !self.options.emit_llvm {
                self.link_binary()?;
                info!("Binary generation completed: {}", self.options.output.display());
            }
        }
        
        info!("Compilation completed successfully");
        Ok(())
    }
    
    /// Parse PHP source code
    pub fn parse(&self) -> CompileResult<Vec<AstNode>> {
        self.parser.parse_file(&self.options.input)
    }
    
    /// Type checking and semantic analysis
    fn type_check(&mut self, ast: &[AstNode]) -> CompileResult<()> {
        info!("Performing type checking and semantic analysis");
        
        for node in ast {
            self.analyze_node(node)?;
        }
        
        Ok(())
    }
    
    /// Analyze a single AST node
    fn analyze_node(&mut self, node: &AstNode) -> CompileResult<()> {
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.analyze_node(stmt)?;
                }
            }
            AstNode::Function(func_decl) => {
                self.analyze_function(func_decl)?;
            }
            AstNode::Class(class_decl) => {
                self.analyze_class(class_decl)?;
            }
            AstNode::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            AstNode::Statement(stmt) => {
                self.analyze_statement(stmt)?;
            }
            _ => {
                // TODO: Implement analysis for other node types
                warn!("Analysis not yet implemented for {:?}", node);
            }
        }
        Ok(())
    }
    
    /// Analyze function declaration
    fn analyze_function(&mut self, func_decl: &crate::ast::FunctionDecl) -> CompileResult<()> {
        // Register function in type context
        let func_type = crate::types::Type::Function(
            func_decl.parameters.iter()
                .map(|p| p.typ.clone().unwrap_or(crate::types::Type::Unknown))
                .collect(),
            Box::new(func_decl.return_type.clone().unwrap_or(crate::types::Type::Unknown))
        );
        
        self.type_context.register_function(func_decl.name.clone(), func_type);
        
        // Analyze function body
        self.analyze_statement(&func_decl.body)?;
        
        Ok(())
    }
    
    /// Analyze class declaration
    fn analyze_class(&mut self, class_decl: &crate::ast::ClassDecl) -> CompileResult<()> {
        let mut class_info = crate::types::ClassInfo::new(class_decl.name.clone());
        
        // Analyze properties
        for prop in &class_decl.properties {
            let prop_type = prop.typ.clone().unwrap_or(crate::types::Type::Unknown);
            class_info.add_property(prop.name.clone(), prop_type);
        }
        
        // Analyze methods
        for method in &class_decl.methods {
            self.analyze_function(method)?;
            let method_type = crate::types::Type::Function(
                method.parameters.iter()
                    .map(|p| p.typ.clone().unwrap_or(crate::types::Type::Unknown))
                    .collect(),
                Box::new(method.return_type.clone().unwrap_or(crate::types::Type::Unknown))
            );
            class_info.add_method(method.name.clone(), method_type);
        }
        
        // Register class in type context
        self.type_context.register_class(class_decl.name.clone(), class_info);
        
        Ok(())
    }
    
    /// Analyze expression
    fn analyze_expression(&self, expr: &crate::ast::Expression) -> CompileResult<()> {
        // TODO: Implement expression analysis
        match expr {
            crate::ast::Expression::Literal(_) => {
                // Literals are always valid
            }
            crate::ast::Expression::Variable(name) => {
                // Check if variable is declared
                if self.type_context.get_variable_type(name).is_none() {
                    warn!("Variable '{}' may be undefined", name);
                }
            }
            _ => {
                // TODO: Implement analysis for other expression types
                warn!("Expression analysis not yet implemented for {:?}", expr);
            }
        }
        Ok(())
    }
    
    /// Analyze statement
    fn analyze_statement(&self, stmt: &crate::ast::Statement) -> CompileResult<()> {
        // TODO: Implement statement analysis
        match stmt {
            crate::ast::Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            crate::ast::Statement::Block(statements) => {
                for stmt in statements {
                    self.analyze_statement(stmt)?;
                }
            }
            _ => {
                // TODO: Implement analysis for other statement types
                warn!("Statement analysis not yet implemented for {:?}", stmt);
            }
        }
        Ok(())
    }
    
    /// Generate LLVM IR
    pub fn generate_ir(&mut self) -> CompileResult<String> {
        let ast = self.parse()?;
        self.ir_generator.generate(&ast)
    }
    
    /// Optimize LLVM IR
    fn optimize_ir(&self, ir: &str) -> CompileResult<()> {
        info!("Optimizing LLVM IR with level {}", self.options.optimization_level);
        
        // TODO: Implement IR optimization passes
        // This would typically involve running LLVM optimization passes
        
        Ok(())
    }
    
    /// Write IR to file
    fn write_ir_file(&self, ir: &str) -> CompileResult<()> {
        let output_path = if self.options.output.extension().is_some() {
            self.options.output.clone()
        } else {
            self.options.output.with_extension("ll")
        };
        
        std::fs::write(&output_path, ir)
            .map_err(|e| CompileError::Io(e))?;
        
        Ok(())
    }
    
    /// Generate object file from IR
    fn generate_object_file(&self, ir: &str) -> CompileResult<()> {
        info!("Generating object file");
        
        let ir_file = self.options.output.with_extension("ll");
        let obj_file = self.options.output.with_extension("o");
        
        // Write IR to temporary file
        std::fs::write(&ir_file, ir)
            .map_err(|e| CompileError::Io(e))?;
        
        // Use llc to generate object file
        let mut cmd = Command::new("llc");
        cmd.arg("-filetype=obj")
            .arg("-o")
            .arg(&obj_file)
            .arg(&ir_file);
        
        if self.options.optimization_level != "O0" {
            cmd.arg(format!("-O{}", &self.options.optimization_level[1..]));
        }
        
        let output = cmd.output()
            .map_err(|e| CompileError::Internal(format!("Failed to run llc: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CompileError::LlvmCompilation(stderr.to_string()));
        }
        
        info!("Object file generated: {}", obj_file.display());
        Ok(())
    }
    
    /// Link binary from object file
    fn link_binary(&self) -> CompileResult<()> {
        info!("Linking binary");
        
        let obj_file = self.options.output.with_extension("o");
        
        // Use lld to link binary
        let mut cmd = Command::new("ld.lld");
        cmd.arg("-o")
            .arg(&self.options.output)
            .arg(&obj_file);
        
        // Add runtime library if not disabled
        if !self.options.no_runtime {
            // TODO: Add runtime library linking
        }
        
        let output = cmd.output()
            .map_err(|e| CompileError::Internal(format!("Failed to run lld: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CompileError::Linking(stderr.to_string()));
        }
        
        info!("Binary linked: {}", self.options.output.display());
        Ok(())
    }
    
    /// Get compiler version information
    pub fn version() -> String {
        format!("php2ir v{}", crate::VERSION)
    }
    
    /// Get supported targets
    pub fn supported_targets() -> Vec<&'static str> {
        vec![
            "x86_64-unknown-linux-gnu",
            "x86_64-apple-darwin",
            "x86_64-pc-windows-gnu",
            "aarch64-unknown-linux-gnu",
            "aarch64-apple-darwin",
        ]
    }
    
    /// Check if target is supported
    pub fn is_target_supported(target: &str) -> bool {
        Self::supported_targets().contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_compiler_options_default() {
        let options = CompilerOptions::default();
        assert_eq!(options.optimization_level, "O2");
        assert!(!options.emit_llvm);
        assert!(!options.emit_llvm_only);
    }

    #[test]
    fn test_compiler_new() {
        let options = CompilerOptions::default();
        let compiler = Compiler::new(options);
        assert!(compiler.is_ok());
    }

    #[test]
    fn test_supported_targets() {
        let targets = Compiler::supported_targets();
        assert!(targets.contains(&"x86_64-unknown-linux-gnu"));
        assert!(targets.contains(&"x86_64-apple-darwin"));
    }

    #[test]
    fn test_target_support_check() {
        assert!(Compiler::is_target_supported("x86_64-unknown-linux-gnu"));
        assert!(!Compiler::is_target_supported("unsupported-target"));
    }
}
