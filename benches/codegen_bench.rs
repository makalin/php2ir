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

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use php2ir::ir::IrGenerator;
use php2ir::ast::{AstNode, Expression, Statement, Literal};

fn create_simple_ast() -> Vec<AstNode> {
    vec![
        AstNode::Program(vec![
            AstNode::Expression(Box::new(
                Expression::Literal(Literal::String("Hello, World!".to_string()))
            ))
        ])
    ]
}

fn create_function_ast() -> Vec<AstNode> {
    use php2ir::ast::{FunctionDecl, Parameter, Type, Visibility};
    
    vec![
        AstNode::Function(FunctionDecl {
            name: "add".to_string(),
            parameters: vec![
                Parameter {
                    name: "a".to_string(),
                    typ: Some(Type::Int),
                    default_value: None,
                    is_reference: false,
                    is_variadic: false,
                },
                Parameter {
                    name: "b".to_string(),
                    typ: Some(Type::Int),
                    default_value: None,
                    is_reference: false,
                    is_variadic: false,
                }
            ],
            return_type: Some(Type::Int),
            body: Box::new(Statement::Return(Some(Box::new(
                Expression::BinaryOp {
                    left: Box::new(Expression::Variable("a".to_string())),
                    op: php2ir::ast::BinaryOperator::Add,
                    right: Box::new(Expression::Variable("b".to_string())),
                }
            )))),
            attributes: vec![],
            is_static: false,
            visibility: Visibility::Public,
        })
    ]
}

fn create_class_ast() -> Vec<AstNode> {
    use php2ir::ast::{ClassDecl, PropertyDecl, FunctionDecl, Parameter, Type, Visibility};
    
    vec![
        AstNode::Class(ClassDecl {
            name: "Calculator".to_string(),
            extends: None,
            implements: vec![],
            properties: vec![
                PropertyDecl {
                    name: "precision".to_string(),
                    typ: Some(Type::Int),
                    default_value: Some(Expression::Literal(Literal::Int(2))),
                    visibility: Visibility::Private,
                    is_static: false,
                    is_readonly: false,
                }
            ],
            methods: vec![
                FunctionDecl {
                    name: "__construct".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "precision".to_string(),
                            typ: Some(Type::Int),
                            default_value: Some(Expression::Literal(Literal::Int(2))),
                            is_reference: false,
                            is_variadic: false,
                        }
                    ],
                    return_type: None,
                    body: Box::new(Statement::Block(vec![])),
                    attributes: vec![],
                    is_static: false,
                    visibility: Visibility::Public,
                }
            ],
            constants: vec![],
            attributes: vec![],
            is_abstract: false,
            is_final: false,
            is_trait: false,
            is_interface: false,
            is_enum: false,
        })
    ]
}

fn bench_generate_simple_ir(c: &mut Criterion) {
    let mut generator = IrGenerator::new().unwrap();
    let ast = create_simple_ast();
    
    c.bench_function("generate_simple_ir", |b| {
        b.iter(|| {
            generator.generate(black_box(&ast)).unwrap();
        });
    });
}

fn bench_generate_function_ir(c: &mut Criterion) {
    let mut generator = IrGenerator::new().unwrap();
    let ast = create_function_ast();
    
    c.bench_function("generate_function_ir", |b| {
        b.iter(|| {
            generator.generate(black_box(&ast)).unwrap();
        });
    });
}

fn bench_generate_class_ir(c: &mut Criterion) {
    let mut generator = IrGenerator::new().unwrap();
    let ast = create_class_ast();
    
    c.bench_function("generate_class_ir", |b| {
        b.iter(|| {
            generator.generate(black_box(&ast)).unwrap();
        });
    });
}

fn bench_generate_large_ir(c: &mut Criterion) {
    let mut generator = IrGenerator::new().unwrap();
    
    // Create a large AST with many functions
    let mut ast = vec![];
    for i in 0..50 {
        use php2ir::ast::{FunctionDecl, Parameter, Type, Visibility, Statement};
        
        ast.push(AstNode::Function(FunctionDecl {
            name: format!("func_{}", i),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    typ: Some(Type::Int),
                    default_value: None,
                    is_reference: false,
                    is_variadic: false,
                }
            ],
            return_type: Some(Type::Int),
            body: Box::new(Statement::Return(Some(Box::new(
                Expression::BinaryOp {
                    left: Box::new(Expression::Variable("x".to_string())),
                    op: php2ir::ast::BinaryOperator::Mul,
                    right: Box::new(Expression::Literal(Literal::Int(i as i64))),
                }
            )))),
            attributes: vec![],
            is_static: false,
            visibility: Visibility::Public,
        }));
    }
    
    c.bench_function("generate_large_ir", |b| {
        b.iter(|| {
            generator.generate(black_box(&ast)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_generate_simple_ir,
    bench_generate_function_ir,
    bench_generate_class_ir,
    bench_generate_large_ir
);
criterion_main!(benches);
