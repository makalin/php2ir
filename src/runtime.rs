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
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_long, c_double, c_void};
use std::ptr;
use log::info;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Garbage collection mode
    pub gc_mode: GcMode,
    
    /// Small string optimization threshold
    pub sso_threshold: usize,
    
    /// Hash policy for associative arrays
    pub hash_policy: HashPolicy,
    
    /// Memory allocation strategy
    pub alloc_strategy: AllocStrategy,
    
    /// Error handling mode
    pub error_mode: ErrorMode,
}

/// Garbage collection modes
#[derive(Debug, Clone, PartialEq)]
pub enum GcMode {
    /// Reference counting (default)
    ReferenceCounting,
    
    /// Boehm GC
    BoehmGc,
    
    /// Mark and sweep
    MarkAndSweep,
    
    /// No GC (manual management)
    None,
}

/// Hash policy for associative arrays
#[derive(Debug, Clone, PartialEq)]
pub enum HashPolicy {
    /// Robin Hood hashing
    RobinHood,
    
    /// Linear probing
    LinearProbing,
    
    /// Quadratic probing
    QuadraticProbing,
}

/// Memory allocation strategy
#[derive(Debug, Clone, PartialEq)]
pub enum AllocStrategy {
    /// System malloc/free
    System,
    
    /// Pool allocator
    Pool,
    
    /// Arena allocator
    Arena,
}

/// Error handling mode
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorMode {
    /// Throw exceptions
    Exceptions,
    
    /// Return error codes
    ErrorCodes,
    
    /// Abort on error
    Abort,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            gc_mode: GcMode::ReferenceCounting,
            sso_threshold: 23,
            hash_policy: HashPolicy::RobinHood,
            alloc_strategy: AllocStrategy::System,
            error_mode: ErrorMode::Exceptions,
        }
    }
}

/// Runtime context
pub struct RuntimeContext {
    config: RuntimeConfig,
    globals: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    classes: HashMap<String, Class>,
    error_handler: Option<Box<dyn Fn(RuntimeError)>>,
}

/// Class implementation
#[derive(Debug, Clone)]
pub struct Class {
    /// Class name
    pub name: String,
    
    /// Parent class
    pub parent: Option<String>,
    
    /// Interfaces
    pub interfaces: Vec<String>,
    
    /// Properties
    pub properties: HashMap<String, Type>,
    
    /// Methods
    pub methods: HashMap<String, Function>,
}

/// Runtime value
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Array),
    Object(Object),
    Resource(Resource),
}

/// Array implementation
#[derive(Debug, Clone)]
pub struct Array {
    /// Array data
    data: Vec<Value>,
    
    /// Hash map for associative arrays
    map: Option<HashMap<String, usize>>,
    
    /// Array type
    array_type: ArrayType,
}

/// Array type
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayType {
    /// Packed array (numeric indices)
    Packed,
    
    /// Associative array (string keys)
    Associative,
    
    /// Mixed array
    Mixed,
}

/// Object implementation
#[derive(Debug, Clone)]
pub struct Object {
    /// Class name
    class_name: String,
    
    /// Properties
    properties: HashMap<String, Value>,
    
    /// Methods
    methods: HashMap<String, Function>,
}

/// Function implementation
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name
    name: String,
    
    /// Parameter types
    param_types: Vec<Type>,
    
    /// Return type
    return_type: Type,
    
    /// Function pointer
    func_ptr: fn(&[Value]) -> Result<Value, RuntimeError>,
}

/// Type information
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Null,
    Bool,
    Int,
    Float,
    String,
    Array,
    Object,
    Resource,
    Mixed,
}

/// Resource handle
#[derive(Debug, Clone)]
pub struct Resource {
    /// Resource type
    resource_type: String,
    
    /// Resource data
    data: Box<dyn std::any::Any>,
    
    /// Resource ID
    id: u64,
}

/// Runtime error
#[derive(Debug, Clone)]
pub struct RuntimeError {
    /// Error message
    pub message: String,
    
    /// Error code
    pub code: i32,
    
    /// Error location
    pub location: Option<String>,
    
    /// Error type
    pub error_type: RuntimeErrorType,
}

/// Runtime error types
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeErrorType {
    /// Type error
    TypeError,
    
    /// Undefined variable
    UndefinedVariable,
    
    /// Undefined function
    UndefinedFunction,
    
    /// Undefined class
    UndefinedClass,
    
    /// Division by zero
    DivisionByZero,
    
    /// Out of memory
    OutOfMemory,
    
    /// Invalid operation
    InvalidOperation,
}

impl RuntimeContext {
    /// Create new runtime context
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            globals: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            error_handler: None,
        }
    }
    
    /// Initialize runtime
    pub fn init(&mut self) -> Result<(), RuntimeError> {
        info!("Initializing PHP runtime");
        
        // Register built-in functions
        self.register_builtin_functions()?;
        
        // Register built-in classes
        self.register_builtin_classes()?;
        
        // Initialize memory management
        self.init_memory_management()?;
        
        // Initialize error handling
        self.init_error_handling()?;
        
        info!("PHP runtime initialized successfully");
        Ok(())
    }
    
    /// Cleanup runtime
    pub fn cleanup(&mut self) -> Result<(), RuntimeError> {
        info!("Cleaning up PHP runtime");
        
        // Cleanup memory
        self.cleanup_memory()?;
        
        // Clear globals
        self.globals.clear();
        
        // Clear functions
        self.functions.clear();
        
        // Clear classes
        self.classes.clear();
        
        info!("PHP runtime cleanup completed");
        Ok(())
    }
    
    /// Register built-in functions
    fn register_builtin_functions(&mut self) -> Result<(), RuntimeError> {
        // String functions
        self.register_function("strlen", vec![Type::String], Type::Int, |args| {
            if let Some(Value::String(s)) = args.get(0) {
                Ok(Value::Int(s.len() as i64))
            } else {
                Err(RuntimeError {
                    message: "strlen() expects string parameter".to_string(),
                    code: -1,
                    location: None,
                    error_type: RuntimeErrorType::TypeError,
                })
            }
        })?;
        
        // Array functions
        self.register_function("count", vec![Type::Array], Type::Int, |args| {
            if let Some(Value::Array(arr)) = args.get(0) {
                Ok(Value::Int(arr.len() as i64))
            } else {
                Err(RuntimeError {
                    message: "count() expects array parameter".to_string(),
                    code: -1,
                    location: None,
                    error_type: RuntimeErrorType::TypeError,
                })
            }
        })?;
        
        // Math functions
        self.register_function("abs", vec![Type::Mixed], Type::Mixed, |args| {
            if let Some(value) = args.get(0) {
                match value {
                    Value::Int(n) => Ok(Value::Int(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(RuntimeError {
                        message: "abs() expects numeric parameter".to_string(),
                        code: -1,
                        location: None,
                        error_type: RuntimeErrorType::TypeError,
                    }),
                }
            } else {
                Err(RuntimeError {
                    message: "abs() expects 1 parameter".to_string(),
                    code: -1,
                    location: None,
                    error_type: RuntimeErrorType::InvalidOperation,
                })
            }
        })?;
        
        Ok(())
    }
    
    /// Register built-in classes
    fn register_builtin_classes(&mut self) -> Result<(), RuntimeError> {
        // TODO: Implement built-in class registration
        Ok(())
    }
    
    /// Initialize memory management
    fn init_memory_management(&mut self) -> Result<(), RuntimeError> {
        match self.config.gc_mode {
            GcMode::BoehmGc => {
                // Initialize Boehm GC
                unsafe {
                    // TODO: Call Boehm GC initialization
                }
            }
            GcMode::MarkAndSweep => {
                // Initialize mark and sweep GC
                // TODO: Implement mark and sweep GC
            }
            _ => {
                // Reference counting or no GC - no initialization needed
            }
        }
        Ok(())
    }
    
    /// Initialize error handling
    fn init_error_handling(&mut self) -> Result<(), RuntimeError> {
        // Set default error handler
        self.error_handler = Some(Box::new(|error| {
            eprintln!("Runtime error: {} (code: {})", error.message, error.code);
            if let Some(location) = &error.location {
                eprintln!("Location: {}", location);
            }
        }));
        Ok(())
    }
    
    /// Cleanup memory
    fn cleanup_memory(&mut self) -> Result<(), RuntimeError> {
        match self.config.gc_mode {
            GcMode::BoehmGc => {
                // Cleanup Boehm GC
                unsafe {
                    // TODO: Call Boehm GC cleanup
                }
            }
            GcMode::MarkAndSweep => {
                // Cleanup mark and sweep GC
                // TODO: Implement mark and sweep GC cleanup
            }
            _ => {
                // Reference counting or no GC - no cleanup needed
            }
        }
        Ok(())
    }
    
    /// Register a function
    pub fn register_function(
        &mut self,
        name: &str,
        param_types: Vec<Type>,
        return_type: Type,
        func: fn(&[Value]) -> Result<Value, RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let function = Function {
            name: name.to_string(),
            param_types,
            return_type,
            func_ptr: func,
        };
        
        self.functions.insert(name.to_string(), function);
        Ok(())
    }
    
    /// Call a function
    pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value, RuntimeError> {
        if let Some(function) = self.functions.get(name) {
            // Check parameter count
            if args.len() != function.param_types.len() {
                return Err(RuntimeError {
                    message: format!("{}() expects {} parameters, got {}", 
                        name, function.param_types.len(), args.len()),
                    code: -1,
                    location: None,
                    error_type: RuntimeErrorType::InvalidOperation,
                });
            }
            
            // Check parameter types
            for (i, (arg, expected_type)) in args.iter().zip(function.param_types.iter()).enumerate() {
                if !self.is_type_compatible(arg, expected_type) {
                    return Err(RuntimeError {
                        message: format!("{}() parameter {} expects {:?}, got {:?}", 
                            name, i + 1, expected_type, self.get_value_type(arg)),
                        code: -1,
                        location: None,
                        error_type: RuntimeErrorType::TypeError,
                    });
                }
            }
            
            // Call function
            (function.func_ptr)(args)
        } else {
            Err(RuntimeError {
                message: format!("Call to undefined function {}", name),
                code: -1,
                location: None,
                error_type: RuntimeErrorType::UndefinedFunction,
            })
        }
    }
    
    /// Check if value is compatible with type
    fn is_type_compatible(&self, value: &Value, typ: &Type) -> bool {
        match (value, typ) {
            (Value::Null, Type::Null) => true,
            (Value::Bool(_), Type::Bool) => true,
            (Value::Int(_), Type::Int) => true,
            (Value::Float(_), Type::Float) => true,
            (Value::String(_), Type::String) => true,
            (Value::Array(_), Type::Array) => true,
            (Value::Object(_), Type::Object) => true,
            (Value::Resource(_), Type::Resource) => true,
            (_, Type::Mixed) => true,
            (Value::Null, _) => true, // Null is compatible with any type
            _ => false,
        }
    }
    
    /// Get value type
    fn get_value_type(&self, value: &Value) -> Type {
        match value {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Array(_) => Type::Array,
            Value::Object(_) => Type::Object,
            Value::Resource(_) => Type::Resource,
        }
    }
    
    /// Set global variable
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.globals.insert(name.to_string(), value);
    }
    
    /// Get global variable
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }
    
    /// Print value
    pub fn print(&self, value: &Value) -> Result<(), RuntimeError> {
        match value {
            Value::Null => print!("null"),
            Value::Bool(b) => print!("{}", b),
            Value::Int(n) => print!("{}", n),
            Value::Float(f) => print!("{}", f),
            Value::String(s) => print!("{}", s),
            Value::Array(arr) => {
                print!("Array");
                // TODO: Implement array printing
            }
            Value::Object(obj) => {
                print!("{} Object", obj.class_name);
            }
            Value::Resource(res) => {
                print!("Resource id #{}", res.id);
            }
        }
        Ok(())
    }
    
    /// Print line
    pub fn println(&self, value: &Value) -> Result<(), RuntimeError> {
        self.print(value)?;
        println!();
        Ok(())
    }
}

impl Array {
    /// Create new array
    pub fn new(array_type: ArrayType) -> Self {
        Self {
            data: Vec::new(),
            map: match array_type {
                ArrayType::Associative | ArrayType::Mixed => Some(HashMap::new()),
                ArrayType::Packed => None,
            },
            array_type,
        }
    }
    
    /// Get array length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Push value to array
    pub fn push(&mut self, value: Value) {
        self.data.push(value);
    }
    
    /// Get value by index
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.data.get(index)
    }
    
    /// Set value by index
    pub fn set(&mut self, index: usize, value: Value) -> Result<(), RuntimeError> {
        if index >= self.data.len() {
            return Err(RuntimeError {
                message: format!("Array index {} out of bounds", index),
                code: -1,
                location: None,
                error_type: RuntimeErrorType::InvalidOperation,
            });
        }
        self.data[index] = value;
        Ok(())
    }
    
    /// Get value by key (for associative arrays)
    pub fn get_by_key(&self, key: &str) -> Option<&Value> {
        if let Some(ref map) = self.map {
            if let Some(&index) = map.get(key) {
                return self.data.get(index);
            }
        }
        None
    }
    
    /// Set value by key (for associative arrays)
    pub fn set_by_key(&mut self, key: &str, value: Value) -> Result<(), RuntimeError> {
        if let Some(ref mut map) = self.map {
            if let Some(&index) = map.get(key) {
                if index < self.data.len() {
                    self.data[index] = value;
                    return Ok(());
                }
            }
            
            // Add new key-value pair
            let index = self.data.len();
            self.data.push(value);
            map.insert(key.to_string(), index);
            Ok(())
        } else {
            Err(RuntimeError {
                message: "Cannot set key on packed array".to_string(),
                code: -1,
                location: None,
                error_type: RuntimeErrorType::InvalidOperation,
            })
        }
    }
}

impl Object {
    /// Create new object
    pub fn new(class_name: String) -> Self {
        Self {
            class_name,
            properties: HashMap::new(),
            methods: HashMap::new(),
        }
    }
    
    /// Set property
    pub fn set_property(&mut self, name: &str, value: Value) {
        self.properties.insert(name.to_string(), value);
    }
    
    /// Get property
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }
    
    /// Add method
    pub fn add_method(&mut self, name: &str, method: Function) {
        self.methods.insert(name.to_string(), method);
    }
    
    /// Get method
    pub fn get_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }
}

impl Resource {
    /// Create new resource
    pub fn new(resource_type: String, data: Box<dyn std::any::Any>) -> Self {
        static mut NEXT_ID: u64 = 1;
        
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };
        
        Self {
            resource_type,
            data,
            id,
        }
    }
    
    /// Get resource type
    pub fn get_type(&self) -> &str {
        &self.resource_type
    }
    
    /// Get resource data
    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
    
    /// Get resource ID
    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl RuntimeError {
    /// Create new runtime error
    pub fn new(message: String, error_type: RuntimeErrorType) -> Self {
        Self {
            message,
            code: -1,
            location: None,
            error_type,
        }
    }
    
    /// Set error code
    pub fn with_code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }
    
    /// Set error location
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(location) = &self.location {
            write!(f, " at {}", location)?;
        }
        Ok(())
    }
}

impl std::error::Error for RuntimeError {}

// FFI functions for C interop
#[no_mangle]
pub extern "C" fn php_runtime_init() -> c_int {
    // TODO: Implement C interop
    0
}

#[no_mangle]
pub extern "C" fn php_runtime_cleanup() -> c_int {
    // TODO: Implement C interop
    0
}

#[no_mangle]
pub extern "C" fn php_print_string(s: *const c_char) -> c_int {
    // TODO: Implement C interop
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_config_default() {
        let config = RuntimeConfig::default();
        assert_eq!(config.gc_mode, GcMode::ReferenceCounting);
        assert_eq!(config.sso_threshold, 23);
        assert_eq!(config.hash_policy, HashPolicy::RobinHood);
    }

    #[test]
    fn test_runtime_context_new() {
        let config = RuntimeConfig::default();
        let context = RuntimeContext::new(config);
        assert!(context.functions.is_empty());
        assert!(context.globals.is_empty());
    }

    #[test]
    fn test_array_operations() {
        let mut array = Array::new(ArrayType::Packed);
        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
        
        array.push(Value::Int(42));
        assert_eq!(array.len(), 1);
        assert!(!array.is_empty());
        
        assert_eq!(array.get(0), Some(&Value::Int(42)));
        array.set(0, Value::Int(100)).unwrap();
        assert_eq!(array.get(0), Some(&Value::Int(100)));
    }

    #[test]
    fn test_object_operations() {
        let mut obj = Object::new("TestClass".to_string());
        
        obj.set_property("x", Value::Int(42));
        assert_eq!(obj.get_property("x"), Some(&Value::Int(42)));
        assert_eq!(obj.get_property("y"), None);
    }

    #[test]
    fn test_type_compatibility() {
        let config = RuntimeConfig::default();
        let context = RuntimeContext::new(config);
        
        assert!(context.is_type_compatible(&Value::Int(42), &Type::Int));
        assert!(context.is_type_compatible(&Value::Int(42), &Type::Mixed));
        assert!(context.is_type_compatible(&Value::Null, &Type::Int));
        assert!(!context.is_type_compatible(&Value::String("hello".to_string()), &Type::Int));
    }
}
