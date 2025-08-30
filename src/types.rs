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
use std::collections::HashMap;

/// PHP type representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Scalar types
    Int,
    Float,
    Bool,
    String,
    
    /// Array types
    Array(Box<Type>),
    AssociativeArray(Box<Type>),
    
    /// Object types
    Object(String), // Class name
    Null,
    
    /// Function types
    Function(Vec<Type>, Box<Type>), // Parameters, return type
    
    /// Union types
    Union(Vec<Type>),
    
    /// Generic types
    Generic(String, Vec<Type>),
    
    /// Unknown type
    Unknown,
}

impl Type {
    /// Check if type is scalar
    pub fn is_scalar(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Bool | Type::String)
    }
    
    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }
    
    /// Check if type is array
    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array(_) | Type::AssociativeArray(_))
    }
    
    /// Check if type is object
    pub fn is_object(&self) -> bool {
        matches!(self, Type::Object(_))
    }
    
    /// Get the element type of an array
    pub fn element_type(&self) -> Option<&Type> {
        match self {
            Type::Array(element_type) | Type::AssociativeArray(element_type) => {
                Some(element_type)
            }
            _ => None,
        }
    }
    
    /// Check if type can be null
    pub fn can_be_null(&self) -> bool {
        matches!(self, Type::Null | Type::Union(types) if types.contains(&Type::Null))
    }
    
    /// Get the underlying type (remove null from union)
    pub fn non_null_type(&self) -> Option<Type> {
        match self {
            Type::Union(types) => {
                let non_null: Vec<Type> = types.iter()
                    .filter(|t| **t != Type::Null)
                    .cloned()
                    .collect();
                match non_null.len() {
                    0 => None,
                    1 => Some(non_null[0].clone()),
                    _ => Some(Type::Union(non_null)),
                }
            }
            Type::Null => None,
            _ => Some(self.clone()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Array(element_type) => write!(f, "array<{}>", element_type),
            Type::AssociativeArray(element_type) => write!(f, "assoc_array<{}>", element_type),
            Type::Object(class_name) => write!(f, "{}", class_name),
            Type::Null => write!(f, "null"),
            Type::Function(params, return_type) => {
                write!(f, "function(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Union(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Generic(name, params) => {
                write!(f, "{}<", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ">")
            }
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

/// PHP value representation
#[derive(Debug, Clone)]
pub enum Value {
    /// Scalar values
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    
    /// Array values
    Array(Vec<Value>),
    AssociativeArray(HashMap<String, Value>),
    
    /// Object values
    Object(String, HashMap<String, Value>), // Class name, properties
    
    /// Null value
    Null,
    
    /// Function value
    Function(String, Vec<Type>, Box<Type>), // Name, parameters, return type
    
    /// Undefined value
    Undefined,
}

impl Value {
    /// Get the type of this value
    pub fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(_) => Type::Array(Box::new(Type::Unknown)),
            Value::AssociativeArray(_) => Type::AssociativeArray(Box::new(Type::Unknown)),
            Value::Object(class_name, _) => Type::Object(class_name.clone()),
            Value::Null => Type::Null,
            Value::Function(_, params, return_type) => {
                Type::Function(params.clone(), return_type.clone())
            }
            Value::Undefined => Type::Unknown,
        }
    }
    
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::AssociativeArray(map) => !map.is_empty(),
            Value::Object(_, _) => true,
            Value::Null => false,
            Value::Function(_, _, _) => true,
            Value::Undefined => false,
        }
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => format!("[{}]", arr.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")),
            Value::AssociativeArray(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{} => {}", k, v.to_string()))
                    .collect();
                format!("[{}]", pairs.join(", "))
            }
            Value::Object(class_name, _) => format!("{} object", class_name),
            Value::Null => "null".to_string(),
            Value::Function(name, _, _) => format!("function {}", name),
            Value::Undefined => "undefined".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Type context for tracking types during compilation
#[derive(Debug, Default)]
pub struct TypeContext {
    types: HashMap<String, Type>,
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    classes: HashMap<String, ClassInfo>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a type alias
    pub fn register_type(&mut self, name: String, typ: Type) {
        self.types.insert(name, typ);
    }
    
    /// Get a type by name
    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }
    
    /// Register a variable type
    pub fn register_variable(&mut self, name: String, typ: Type) {
        self.variables.insert(name, typ);
    }
    
    /// Get variable type
    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }
    
    /// Register a function signature
    pub fn register_function(&mut self, name: String, typ: Type) {
        self.functions.insert(name, typ);
    }
    
    /// Get function type
    pub fn get_function_type(&self, name: &str) -> Option<&Type> {
        self.functions.get(name)
    }
    
    /// Register a class
    pub fn register_class(&mut self, name: String, info: ClassInfo) {
        self.classes.insert(name, info);
    }
    
    /// Get class info
    pub fn get_class_info(&self, name: &str) -> Option<&ClassInfo> {
        self.classes.get(name)
    }
}

/// Class information
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub properties: HashMap<String, Type>,
    pub methods: HashMap<String, Type>,
    pub parent: Option<String>,
    pub interfaces: Vec<String>,
}

impl ClassInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: HashMap::new(),
            methods: HashMap::new(),
            parent: None,
            interfaces: Vec::new(),
        }
    }
    
    pub fn add_property(&mut self, name: String, typ: Type) {
        self.properties.insert(name, typ);
    }
    
    pub fn add_method(&mut self, name: String, typ: Type) {
        self.methods.insert(name, typ);
    }
    
    pub fn set_parent(&mut self, parent: String) {
        self.parent = Some(parent);
    }
    
    pub fn add_interface(&mut self, interface: String) {
        self.interfaces.push(interface);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int.to_string(), "int");
        assert_eq!(Type::Float.to_string(), "float");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::String.to_string(), "string");
        assert_eq!(Type::Array(Box::new(Type::Int)).to_string(), "array<int>");
        assert_eq!(Type::Object("MyClass".to_string()).to_string(), "MyClass");
    }

    #[test]
    fn test_value_type() {
        assert_eq!(Value::Int(42).get_type(), Type::Int);
        assert_eq!(Value::String("hello".to_string()).get_type(), Type::String);
        assert_eq!(Value::Null.get_type(), Type::Null);
    }

    #[test]
    fn test_value_truthiness() {
        assert!(Value::Int(1).is_truthy());
        assert!(!Value::Int(0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
        assert!(!Value::Null.is_truthy());
    }

    #[test]
    fn test_type_context() {
        let mut ctx = TypeContext::new();
        ctx.register_type("MyType".to_string(), Type::Int);
        ctx.register_variable("x".to_string(), Type::String);
        
        assert_eq!(ctx.get_type("MyType"), Some(&Type::Int));
        assert_eq!(ctx.get_variable_type("x"), Some(&Type::String));
        assert_eq!(ctx.get_type("Unknown"), None);
    }
}
