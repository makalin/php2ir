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
use crate::types::Type;

/// PHP AST node
#[derive(Debug, Clone)]
pub enum AstNode {
    /// Program root
    Program(Vec<AstNode>),
    
    /// Statements
    Expression(Box<Expression>),
    Statement(Box<Statement>),
    
    /// Declarations
    Function(FunctionDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Trait(TraitDecl),
    Enum(EnumDecl),
    
    /// Namespace
    Namespace(NamespaceDecl),
    Use(UseDecl),
    
    /// Attributes
    Attribute(Attribute),
}

/// Expression node
#[derive(Debug, Clone)]
pub enum Expression {
    /// Literals
    Literal(Literal),
    
    /// Variables
    Variable(String),
    VariableVariable(Box<Expression>),
    
    /// Binary operations
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    
    /// Unary operations
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    
    /// Function calls
    FunctionCall {
        name: Box<Expression>,
        arguments: Vec<Expression>,
    },
    
    /// Method calls
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
    
    /// Property access
    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },
    
    /// Array access
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    
    /// Assignment
    Assignment {
        target: Box<Expression>,
        op: AssignmentOperator,
        value: Box<Expression>,
    },
    
    /// Ternary operator
    Ternary {
        condition: Box<Expression>,
        true_expr: Box<Expression>,
        false_expr: Box<Expression>,
    },
    
    /// Null coalescing
    NullCoalescing {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    
    /// Cast
    Cast {
        target_type: Type,
        expr: Box<Expression>,
    },
    
    /// Instanceof
    InstanceOf {
        expr: Box<Expression>,
        class: Box<Expression>,
    },
    
    /// New object
    New {
        class: Box<Expression>,
        arguments: Vec<Expression>,
    },
    
    /// Clone
    Clone(Box<Expression>),
    
    /// Include/require
    Include {
        kind: IncludeKind,
        file: Box<Expression>,
    },
    
    /// Yield
    Yield {
        key: Option<Box<Expression>>,
        value: Option<Box<Expression>>,
    },
    
    /// Array creation
    Array {
        elements: Vec<ArrayElement>,
    },
    
    /// List assignment
    List {
        variables: Vec<Expression>,
    },
}

/// Statement node
#[derive(Debug, Clone)]
pub enum Statement {
    /// Expression statement
    Expression(Box<Expression>),
    
    /// Block statement
    Block(Vec<Statement>),
    
    /// If statement
    If {
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    
    /// While loop
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    
    /// Do-while loop
    DoWhile {
        body: Box<Statement>,
        condition: Box<Expression>,
    },
    
    /// For loop
    For {
        init: Vec<Expression>,
        condition: Vec<Expression>,
        update: Vec<Expression>,
        body: Box<Statement>,
    },
    
    /// Foreach loop
    Foreach {
        array: Box<Expression>,
        key: Option<String>,
        value: String,
        body: Box<Statement>,
    },
    
    /// Switch statement
    Switch {
        expression: Box<Expression>,
        cases: Vec<SwitchCase>,
    },
    
    /// Match expression
    Match {
        expression: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    
    /// Try-catch
    Try {
        try_block: Box<Statement>,
        catch_blocks: Vec<CatchBlock>,
        finally_block: Option<Box<Statement>>,
    },
    
    /// Throw
    Throw(Box<Expression>),
    
    /// Return
    Return(Option<Box<Expression>>),
    
    /// Break
    Break(Option<Box<Expression>>),
    
    /// Continue
    Continue(Option<Box<Expression>>),
    
    /// Global
    Global(Vec<String>),
    
    /// Static
    Static(Vec<String>),
    
    /// Echo
    Echo(Vec<Expression>),
    
    /// Print
    Print(Box<Expression>),
    
    /// Unset
    Unset(Vec<Expression>),
    
    /// Isset
    Isset(Vec<Expression>),
    
    /// Empty
    Empty(Box<Expression>),
    
    /// Die/exit
    Die(Option<Box<Expression>>),
    
    /// Declare
    Declare {
        directives: Vec<DeclareDirective>,
        body: Box<Statement>,
    },
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<ArrayElement>),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Mod,        // %
    Pow,        // **
    Concat,     // .
    Equal,      // ==
    Identical,  // ===
    NotEqual,   // !=
    NotIdentical, // !==
    Less,       // <
    LessEqual,  // <=
    Greater,    // >
    GreaterEqual, // >=
    Spaceship,  // <=>
    And,        // &&
    Or,         // ||
    Xor,        // xor
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    ShiftLeft,  // <<
    ShiftRight, // >>
    Coalesce,   // ??
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,       // +
    Minus,      // -
    Not,        // !
    BitwiseNot, // ~
    PreInc,     // ++
    PreDec,     // --
    PostInc,    // ++
    PostDec,    // --
    ErrorSuppress, // @
}

/// Assignment operators
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,     // =
    AddAssign,  // +=
    SubAssign,  // -=
    MulAssign,  // *=
    DivAssign,  // /=
    ModAssign,  // %=
    PowAssign,  // **=
    ConcatAssign, // .=
    BitwiseAndAssign, // &=
    BitwiseOrAssign,  // |=
    BitwiseXorAssign, // ^=
    ShiftLeftAssign,  // <<=
    ShiftRightAssign, // >>=
    CoalesceAssign,   // ??=
}

/// Function declaration
#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Box<Statement>,
    pub attributes: Vec<Attribute>,
    pub is_static: bool,
    pub visibility: Visibility,
}

/// Class declaration
#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub properties: Vec<PropertyDecl>,
    pub methods: Vec<FunctionDecl>,
    pub constants: Vec<ConstantDecl>,
    pub attributes: Vec<Attribute>,
    pub is_abstract: bool,
    pub is_final: bool,
    pub is_trait: bool,
    pub is_interface: bool,
    pub is_enum: bool,
}

/// Parameter declaration
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: Option<Type>,
    pub default_value: Option<Expression>,
    pub is_reference: bool,
    pub is_variadic: bool,
}

/// Property declaration
#[derive(Debug, Clone)]
pub struct PropertyDecl {
    pub name: String,
    pub typ: Option<Type>,
    pub default_value: Option<Expression>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_readonly: bool,
}

/// Constant declaration
#[derive(Debug, Clone)]
pub struct ConstantDecl {
    pub name: String,
    pub value: Expression,
    pub visibility: Visibility,
}

/// Interface declaration
#[derive(Debug, Clone)]
pub struct InterfaceDecl {
    pub name: String,
    pub extends: Vec<String>,
    pub constants: Vec<ConstantDecl>,
    pub methods: Vec<FunctionDecl>,
}

/// Trait declaration
#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub properties: Vec<PropertyDecl>,
    pub methods: Vec<FunctionDecl>,
    pub constants: Vec<ConstantDecl>,
}

/// Enum declaration
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub backing_type: Option<Type>,
    pub cases: Vec<EnumCase>,
    pub methods: Vec<FunctionDecl>,
}

/// Enum case
#[derive(Debug, Clone)]
pub struct EnumCase {
    pub name: String,
    pub value: Option<Expression>,
}

/// Namespace declaration
#[derive(Debug, Clone)]
pub struct NamespaceDecl {
    pub name: Option<String>,
    pub statements: Vec<AstNode>,
}

/// Use declaration
#[derive(Debug, Clone)]
pub struct UseDecl {
    pub uses: Vec<UseClause>,
    pub kind: UseKind,
}

/// Use clause
#[derive(Debug, Clone)]
pub struct UseClause {
    pub name: String,
    pub alias: Option<String>,
}

/// Use kind
#[derive(Debug, Clone)]
pub enum UseKind {
    Normal,
    Function,
    Const,
}

/// Visibility
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

/// Attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<Expression>,
}

/// Array element
#[derive(Debug, Clone)]
pub struct ArrayElement {
    pub key: Option<Expression>,
    pub value: Expression,
    pub is_reference: bool,
}

/// Switch case
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub condition: Option<Expression>,
    pub statements: Vec<Statement>,
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub patterns: Vec<Expression>,
    pub body: Box<Statement>,
}

/// Catch block
#[derive(Debug, Clone)]
pub struct CatchBlock {
    pub types: Vec<Type>,
    pub variable: Option<String>,
    pub body: Box<Statement>,
}

/// Declare directive
#[derive(Debug, Clone)]
pub struct DeclareDirective {
    pub name: String,
    pub value: Expression,
}

/// Include kind
#[derive(Debug, Clone)]
pub enum IncludeKind {
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::Pow => write!(f, "**"),
            BinaryOperator::Concat => write!(f, "."),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::Identical => write!(f, "==="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::NotIdentical => write!(f, "!=="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::Spaceship => write!(f, "<=>"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Xor => write!(f, "xor"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseXor => write!(f, "^"),
            BinaryOperator::ShiftLeft => write!(f, "<<"),
            BinaryOperator::ShiftRight => write!(f, ">>"),
            BinaryOperator::Coalesce => write!(f, "??"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::BitwiseNot => write!(f, "~"),
            UnaryOperator::PreInc => write!(f, "++"),
            UnaryOperator::PreDec => write!(f, "--"),
            UnaryOperator::PostInc => write!(f, "++"),
            UnaryOperator::PostDec => write!(f, "--"),
            UnaryOperator::ErrorSuppress => write!(f, "@"),
        }
    }
}

impl fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentOperator::Assign => write!(f, "="),
            AssignmentOperator::AddAssign => write!(f, "+="),
            AssignmentOperator::SubAssign => write!(f, "-="),
            AssignmentOperator::MulAssign => write!(f, "*="),
            AssignmentOperator::DivAssign => write!(f, "/="),
            AssignmentOperator::ModAssign => write!(f, "%="),
            AssignmentOperator::PowAssign => write!(f, "**="),
            AssignmentOperator::ConcatAssign => write!(f, ".="),
            AssignmentOperator::BitwiseAndAssign => write!(f, "&="),
            AssignmentOperator::BitwiseOrAssign => write!(f, "|="),
            AssignmentOperator::BitwiseXorAssign => write!(f, "^="),
            AssignmentOperator::ShiftLeftAssign => write!(f, "<<="),
            AssignmentOperator::ShiftRightAssign => write!(f, ">>="),
            AssignmentOperator::CoalesceAssign => write!(f, "??="),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Protected => write!(f, "protected"),
            Visibility::Private => write!(f, "private"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_operator_display() {
        assert_eq!(BinaryOperator::Add.to_string(), "+");
        assert_eq!(BinaryOperator::Equal.to_string(), "==");
        assert_eq!(BinaryOperator::And.to_string(), "&&");
    }

    #[test]
    fn test_unary_operator_display() {
        assert_eq!(UnaryOperator::Plus.to_string(), "+");
        assert_eq!(UnaryOperator::Not.to_string(), "!");
        assert_eq!(UnaryOperator::PreInc.to_string(), "++");
    }

    #[test]
    fn test_visibility_display() {
        assert_eq!(Visibility::Public.to_string(), "public");
        assert_eq!(Visibility::Protected.to_string(), "protected");
        assert_eq!(Visibility::Private.to_string(), "private");
    }
}
