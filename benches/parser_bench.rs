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
use php2ir::parser::{DefaultParser, Parser};

fn bench_parse_simple(c: &mut Criterion) {
    let parser = DefaultParser::new();
    let source = r#"
        <?php
        function hello($name) {
            return "Hello, " . $name;
        }
        
        echo hello("World");
    "#;
    
    c.bench_function("parse_simple", |b| {
        b.iter(|| {
            parser.parse(black_box(source)).unwrap();
        });
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    let parser = DefaultParser::new();
    let source = r#"
        <?php
        namespace Test;
        
        use Test\Classes\Person;
        
        class Calculator {
            private $precision;
            
            public function __construct(int $precision = 2) {
                $this->precision = $precision;
            }
            
            public function add(float $a, float $b): float {
                return round($a + $b, $this->precision);
            }
            
            public function multiply(float $a, float $b): float {
                return round($a * $b, $this->precision);
            }
        }
        
        $calc = new Calculator(3);
        $result = $calc->add(3.14159, 2.71828);
        echo "Result: " . $result;
    "#;
    
    c.bench_function("parse_complex", |b| {
        b.iter(|| {
            parser.parse(black_box(source)).unwrap();
        });
    });
}

fn bench_parse_large(c: &mut Criterion) {
    let parser = DefaultParser::new();
    
    // Generate a large PHP source with many functions
    let mut source = String::from("<?php\n");
    
    for i in 0..100 {
        source.push_str(&format!(
            "function func_{}($x) {{\n    return $x * {};\n}}\n",
            i, i
        ));
    }
    
    c.bench_function("parse_large", |b| {
        b.iter(|| {
            parser.parse(black_box(&source)).unwrap();
        });
    });
}

criterion_group!(benches, bench_parse_simple, bench_parse_complex, bench_parse_large);
criterion_main!(benches);
