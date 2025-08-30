<?php
/**
 * Hello World example for php2ir compiler
 * 
 * This demonstrates basic PHP syntax that the compiler supports:
 * - Function definitions
 * - String literals
 * - Function calls
 * - Echo statements
 */

function greet(string $name): string {
    return "Hello, $name!";
}

function get_greeting(): string {
    return greet("World");
}

// Main program
echo get_greeting(), PHP_EOL;
echo "Welcome to php2ir!", PHP_EOL;
echo "PHP → LLVM IR → Native", PHP_EOL;

// Demonstrate some basic operations
$number = 42;
$message = "The answer is: " . $number;
echo $message, PHP_EOL;

// Simple conditional
if ($number > 40) {
    echo "That's a big number!", PHP_EOL;
} else {
    echo "That's a small number.", PHP_EOL;
}

// Simple loop
for ($i = 1; $i <= 3; $i++) {
    echo "Count: $i", PHP_EOL;
}
