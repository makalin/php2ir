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

use std::collections::HashMap;
use log::{info, warn};
use crate::ast::{AstNode, Expression, Statement, Literal, BinaryOperator, UnaryOperator};
use crate::error::{CompileError, CompileResult};
use crate::types::{Type, TypeContext};

/// LLVM IR generator
pub struct IrGenerator {
    /// Type context for type information
    type_context: TypeContext,
    
    /// Current function being generated
    current_function: Option<String>,
    
    /// Variable counter for unique names
    var_counter: u32,
    
    /// Basic block counter
    block_counter: u32,
    
    /// Generated IR code
    ir_code: String,
    
    /// Function declarations
    functions: HashMap<String, FunctionInfo>,
    
    /// Global variables
    globals: HashMap<String, GlobalInfo>,
}

/// Function information
#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    return_type: Type,
    parameters: Vec<ParameterInfo>,
    is_external: bool,
}

/// Parameter information
#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    typ: Type,
    is_reference: bool,
}

/// Global variable information
#[derive(Debug, Clone)]
struct GlobalInfo {
    name: String,
    typ: Type,
    value: Option<String>,
    is_constant: bool,
}

impl IrGenerator {
    /// Create a new IR generator
    pub fn new() -> CompileResult<Self> {
        Ok(Self {
            type_context: TypeContext::new(),
            current_function: None,
            var_counter: 0,
            block_counter: 0,
            ir_code: String::new(),
            functions: HashMap::new(),
            globals: HashMap::new(),
        })
    }
    
    /// Generate LLVM IR from AST
    pub fn generate(&mut self, ast: &[AstNode]) -> CompileResult<String> {
        info!("Generating LLVM IR from {} AST nodes", ast.len());
        
        // Reset state
        self.ir_code.clear();
        self.var_counter = 0;
        self.block_counter = 0;
        
        // Generate module header
        self.generate_module_header()?;
        
        // Generate IR for each AST node
        for node in ast {
            self.generate_node(node)?;
        }
        
        // Generate runtime functions
        self.generate_runtime_functions()?;
        
        // Generate module footer
        self.generate_module_footer()?;
        
        info!("LLVM IR generation completed");
        Ok(self.ir_code.clone())
    }
    
    /// Generate module header
    fn generate_module_header(&mut self) -> CompileResult<()> {
        self.ir_code.push_str("; ModuleID = 'php2ir'\n");
        self.ir_code.push_str("source_filename = \"php2ir\"\n");
        self.ir_code.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        self.ir_code.push_str("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // Declare runtime functions
        self.declare_runtime_functions()?;
        
        Ok(())
    }
    
    /// Generate module footer
    fn generate_module_footer(&mut self) -> CompileResult<()> {
        // Add any final cleanup
        Ok(())
    }
    
    /// Generate IR for a single AST node
    fn generate_node(&mut self, node: &AstNode) -> CompileResult<()> {
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.generate_node(stmt)?;
                }
            }
            AstNode::Function(func_decl) => {
                self.generate_function(func_decl)?;
            }
            AstNode::Class(class_decl) => {
                self.generate_class(class_decl)?;
            }
            AstNode::Expression(expr) => {
                self.generate_expression(expr)?;
            }
            AstNode::Statement(stmt) => {
                self.generate_statement(stmt)?;
            }
            _ => {
                warn!("IR generation not yet implemented for {:?}", node);
            }
        }
        Ok(())
    }
    
    /// Generate function IR
    fn generate_function(&mut self, func_decl: &crate::ast::FunctionDecl) -> CompileResult<()> {
        let func_name = &func_decl.name;
        let return_type = self.llvm_type(&func_decl.return_type.as_ref().unwrap_or(&Type::Unknown));
        
        // Generate function signature
        let params: Vec<String> = func_decl.parameters.iter()
            .map(|p| {
                let param_type = self.llvm_type(&p.typ.as_ref().unwrap_or(&Type::Unknown));
                format!("{} %{}", param_type, p.name)
            })
            .collect();
        
        let param_list = params.join(", ");
        self.ir_code.push_str(&format!("define {} @{}({}) {{\n", return_type, func_name, param_list));
        
        // Set current function context
        self.current_function = Some(func_name.clone());
        
        // Generate function body
        self.generate_statement(&func_decl.body)?;
        
        // Add default return if needed
        if return_type != "void" {
            self.ir_code.push_str(&format!("  ret {} undef\n", return_type));
        }
        
        self.ir_code.push_str("}\n\n");
        
        // Clear current function context
        self.current_function = None;
        
        Ok(())
    }
    
    /// Generate class IR
    fn generate_class(&mut self, class_decl: &crate::ast::ClassDecl) -> CompileResult<()> {
        // TODO: Implement class IR generation
        // This would involve creating struct types and method functions
        warn!("Class IR generation not yet implemented for {}", class_decl.name);
        Ok(())
    }
    
    /// Generate expression IR
    fn generate_expression(&mut self, expr: &Expression) -> CompileResult<()> {
        match expr {
            Expression::Literal(literal) => {
                self.generate_literal(literal)?;
            }
            Expression::Variable(name) => {
                self.generate_variable_access(name)?;
            }
            Expression::BinaryOp { left, op, right } => {
                self.generate_binary_op(left, op, right)?;
            }
            Expression::UnaryOp { op, expr } => {
                self.generate_unary_op(op, expr)?;
            }
            Expression::FunctionCall { name, arguments } => {
                self.generate_function_call(name, arguments)?;
            }
            _ => {
                warn!("Expression IR generation not yet implemented for {:?}", expr);
            }
        }
        Ok(())
    }
    
    /// Generate statement IR
    fn generate_statement(&mut self, stmt: &Statement) -> CompileResult<()> {
        match stmt {
            Statement::Expression(expr) => {
                self.generate_expression(expr)?;
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch)?;
            }
            Statement::While { condition, body } => {
                self.generate_while_loop(condition, body)?;
            }
            Statement::Return(expr) => {
                self.generate_return(expr)?;
            }
            Statement::Echo(expressions) => {
                self.generate_echo(expressions)?;
            }
            _ => {
                warn!("Statement IR generation not yet implemented for {:?}", stmt);
            }
        }
        Ok(())
    }
    
    /// Generate literal IR
    fn generate_literal(&mut self, literal: &Literal) -> CompileResult<()> {
        match literal {
            Literal::Int(n) => {
                let var = self.new_var();
                self.ir_code.push_str(&format!("  {} = add i64 0, {}\n", var, n));
            }
            Literal::Float(x) => {
                let var = self.new_var();
                self.ir_code.push_str(&format!("  {} = fadd double 0.0, {}\n", var, x));
            }
            Literal::String(s) => {
                let var = self.new_var();
                let global_name = self.new_global_string(s);
                self.ir_code.push_str(&format!("  {} = getelementptr [{} x i8], [{} x i8]* @{}, i32 0, i32 0\n", 
                    var, s.len() + 1, s.len() + 1, global_name));
            }
            Literal::Bool(b) => {
                let var = self.new_var();
                let value = if *b { 1 } else { 0 };
                self.ir_code.push_str(&format!("  {} = add i1 0, {}\n", var, value));
            }
            Literal::Null => {
                let var = self.new_var();
                self.ir_code.push_str(&format!("  {} = inttoptr i64 0 to i8*\n", var));
            }
            Literal::Array(_) => {
                // TODO: Implement array literal generation
                warn!("Array literal IR generation not yet implemented");
            }
        }
        Ok(())
    }
    
    /// Generate variable access IR
    fn generate_variable_access(&mut self, name: &str) -> CompileResult<()> {
        // TODO: Implement variable access generation
        // This would involve loading from the appropriate scope
        warn!("Variable access IR generation not yet implemented for {}", name);
        Ok(())
    }
    
    /// Generate binary operation IR
    fn generate_binary_op(&mut self, left: &Expression, op: &BinaryOperator, right: &Expression) -> CompileResult<()> {
        // Generate left and right operands
        self.generate_expression(left)?;
        let left_var = self.last_var();
        
        self.generate_expression(right)?;
        let right_var = self.last_var();
        
        let result_var = self.new_var();
        
        // Generate operation based on operator
        match op {
            BinaryOperator::Add => {
                self.ir_code.push_str(&format!("  {} = add i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Sub => {
                self.ir_code.push_str(&format!("  {} = sub i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Mul => {
                self.ir_code.push_str(&format!("  {} = mul i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Div => {
                self.ir_code.push_str(&format!("  {} = sdiv i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Mod => {
                self.ir_code.push_str(&format!("  {} = srem i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Equal => {
                self.ir_code.push_str(&format!("  {} = icmp eq i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Less => {
                self.ir_code.push_str(&format!("  {} = icmp slt i64 {}, {}\n", result_var, left_var, right_var));
            }
            BinaryOperator::Greater => {
                self.ir_code.push_str(&format!("  {} = icmp sgt i64 {}, {}\n", result_var, left_var, right_var));
            }
            _ => {
                warn!("Binary operator IR generation not yet implemented for {:?}", op);
                self.ir_code.push_str(&format!("  {} = add i64 {}, {}\n", result_var, left_var, right_var));
            }
        }
        
        Ok(())
    }
    
    /// Generate unary operation IR
    fn generate_unary_op(&mut self, op: &UnaryOperator, expr: &Expression) -> CompileResult<()> {
        // Generate operand
        self.generate_expression(expr)?;
        let operand_var = self.last_var();
        
        let result_var = self.new_var();
        
        // Generate operation based on operator
        match op {
            UnaryOperator::Plus => {
                self.ir_code.push_str(&format!("  {} = add i64 0, {}\n", result_var, operand_var));
            }
            UnaryOperator::Minus => {
                self.ir_code.push_str(&format!("  {} = sub i64 0, {}\n", result_var, operand_var));
            }
            UnaryOperator::Not => {
                self.ir_code.push_str(&format!("  {} = icmp eq i1 {}, 0\n", result_var, operand_var));
            }
            _ => {
                warn!("Unary operator IR generation not yet implemented for {:?}", op);
                self.ir_code.push_str(&format!("  {} = add i64 0, {}\n", result_var, operand_var));
            }
        }
        
        Ok(())
    }
    
    /// Generate function call IR
    fn generate_function_call(&mut self, name: &Expression, arguments: &[Expression]) -> CompileResult<()> {
        // TODO: Implement function call generation
        warn!("Function call IR generation not yet implemented");
        Ok(())
    }
    
    /// Generate if statement IR
    fn generate_if_statement(&mut self, condition: &Expression, then_branch: &Statement, else_branch: &Option<Box<Statement>>) -> CompileResult<()> {
        let then_block = self.new_block();
        let else_block = self.new_block();
        let merge_block = self.new_block();
        
        // Generate condition
        self.generate_expression(condition)?;
        let cond_var = self.last_var();
        
        // Generate conditional branch
        self.ir_code.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_var, then_block, else_block));
        
        // Generate then branch
        self.ir_code.push_str(&format!("{}:\n", then_block));
        self.generate_statement(then_branch)?;
        self.ir_code.push_str(&format!("  br label %{}\n", merge_block));
        
        // Generate else branch
        self.ir_code.push_str(&format!("{}:\n", else_block));
        if let Some(else_stmt) = else_branch {
            self.generate_statement(else_stmt)?;
        }
        self.ir_code.push_str(&format!("  br label %{}\n", merge_block));
        
        // Generate merge block
        self.ir_code.push_str(&format!("{}:\n", merge_block));
        
        Ok(())
    }
    
    /// Generate while loop IR
    fn generate_while_loop(&mut self, condition: &Expression, body: &Statement) -> CompileResult<()> {
        let loop_header = self.new_block();
        let loop_body = self.new_block();
        let loop_exit = self.new_block();
        
        // Jump to loop header
        self.ir_code.push_str(&format!("  br label %{}\n", loop_header));
        
        // Loop header - check condition
        self.ir_code.push_str(&format!("{}:\n", loop_header));
        self.generate_expression(condition)?;
        let cond_var = self.last_var();
        self.ir_code.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_var, loop_body, loop_exit));
        
        // Loop body
        self.ir_code.push_str(&format!("{}:\n", loop_body));
        self.generate_statement(body)?;
        self.ir_code.push_str(&format!("  br label %{}\n", loop_header));
        
        // Loop exit
        self.ir_code.push_str(&format!("{}:\n", loop_exit));
        
        Ok(())
    }
    
    /// Generate return statement IR
    fn generate_return(&mut self, expr: &Option<Box<Expression>>) -> CompileResult<()> {
        if let Some(expr) = expr {
            self.generate_expression(expr)?;
            let value_var = self.last_var();
            self.ir_code.push_str(&format!("  ret i64 {}\n", value_var));
        } else {
            self.ir_code.push_str("  ret void\n");
        }
        Ok(())
    }
    
    /// Generate echo statement IR
    fn generate_echo(&mut self, expressions: &[Expression]) -> CompileResult<()> {
        for expr in expressions {
            self.generate_expression(expr)?;
            let value_var = self.last_var();
            
            // Call runtime print function
            self.ir_code.push_str(&format!("  call void @php_print(i8* {})\n", value_var));
        }
        Ok(())
    }
    
    /// Generate runtime functions
    fn generate_runtime_functions(&mut self) -> CompileResult<()> {
        // Main function
        self.ir_code.push_str("define i32 @main(i32 %argc, i8** %argv) {\n");
        self.ir_code.push_str("  call void @php_init()\n");
        
        // TODO: Call the main PHP function
        
        self.ir_code.push_str("  call void @php_cleanup()\n");
        self.ir_code.push_str("  ret i32 0\n");
        self.ir_code.push_str("}\n\n");
        
        Ok(())
    }
    
    /// Declare runtime functions
    fn declare_runtime_functions(&mut self) -> CompileResult<()> {
        self.ir_code.push_str("declare void @php_init()\n");
        self.ir_code.push_str("declare void @php_cleanup()\n");
        self.ir_code.push_str("declare void @php_print(i8*)\n");
        self.ir_code.push_str("declare i8* @php_malloc(i64)\n");
        self.ir_code.push_str("declare void @php_free(i8*)\n\n");
        
        Ok(())
    }
    
    /// Convert PHP type to LLVM type
    fn llvm_type(&self, typ: &Type) -> &'static str {
        match typ {
            Type::Int => "i64",
            Type::Float => "double",
            Type::Bool => "i1",
            Type::String => "i8*",
            Type::Array(_) => "i8*", // Array pointer
            Type::Object(_) => "i8*", // Object pointer
            Type::Null => "i8*",
            Type::Unknown => "i8*",
            _ => "i8*", // Default to generic pointer
        }
    }
    
    /// Generate new variable name
    fn new_var(&mut self) -> String {
        self.var_counter += 1;
        format!("%{}", self.var_counter - 1)
    }
    
    /// Get last generated variable
    fn last_var(&self) -> String {
        format!("%{}", self.var_counter - 1)
    }
    
    /// Generate new basic block name
    fn new_block(&mut self) -> String {
        self.block_counter += 1;
        format!("bb{}", self.block_counter - 1)
    }
    
    /// Generate new global string
    fn new_global_string(&mut self, s: &str) -> String {
        let global_name = format!("@.str.{}", self.var_counter);
        self.var_counter += 1;
        
        // Add global string declaration
        self.ir_code.push_str(&format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", 
            global_name, s.len() + 1, s));
        
        global_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Expression, Literal};

    #[test]
    fn test_ir_generator_new() {
        let generator = IrGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_llvm_type_conversion() {
        let generator = IrGenerator::new().unwrap();
        
        assert_eq!(generator.llvm_type(&Type::Int), "i64");
        assert_eq!(generator.llvm_type(&Type::Float), "double");
        assert_eq!(generator.llvm_type(&Type::Bool), "i1");
        assert_eq!(generator.llvm_type(&Type::String), "i8*");
    }

    #[test]
    fn test_generate_simple_program() {
        let mut generator = IrGenerator::new().unwrap();
        
        let ast = vec![
            AstNode::Program(vec![
                AstNode::Expression(Box::new(
                    Expression::Literal(Literal::Int(42))
                ))
            ])
        ];
        
        let result = generator.generate(&ast);
        assert!(result.is_ok());
        
        let ir = result.unwrap();
        assert!(ir.contains("ModuleID = 'php2ir'"));
        assert!(ir.contains("add i64 0, 42"));
    }
}
