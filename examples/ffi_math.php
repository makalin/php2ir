<?php
/**
 * FFI (Foreign Function Interface) example for php2ir compiler
 * 
 * This demonstrates FFI features that the compiler supports:
 * - Native function declarations
 * - Math library interop
 * - Performance comparison
 */

// FFI declarations for math functions
#[ffi("libm.so.6", "double cos(double)")]
function cos_native(float $x): float {}

#[ffi("libm.so.6", "double sin(double)")]
function sin_native(float $x): float {}

#[ffi("libm.so.6", "double sqrt(double)")]
function sqrt_native(float $x): float {}

#[ffi("libm.so.6", "double pow(double, double)")]
function pow_native(float $x, float $y): float {}

// PHP implementations for comparison
function cos_php(float $x): float {
    return cos($x);
}

function sin_php(float $x): float {
    return sin($x);
}

function sqrt_php(float $x): float {
    return sqrt($x);
}

function pow_php(float $x, float $y): float {
    return $x ** $y;
}

// Test values
$angles = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
$numbers = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0];

echo "FFI Math Function Test", PHP_EOL;
echo "======================", PHP_EOL;

// Test trigonometric functions
echo "Cosine values:", PHP_EOL;
foreach ($angles as $angle) {
    $native_result = cos_native($angle);
    $php_result = cos_php($angle);
    $diff = abs($native_result - $php_result);
    echo "  cos($angle) = $native_result (native), $php_result (PHP), diff: $diff", PHP_EOL;
}

echo PHP_EOL;

// Test square root
echo "Square root values:", PHP_EOL;
foreach ($numbers as $num) {
    $native_result = sqrt_native($num);
    $php_result = sqrt_php($num);
    $diff = abs($native_result - $php_result);
    echo "  sqrt($num) = $native_result (native), $php_result (PHP), diff: $diff", PHP_EOL;
}

echo PHP_EOL;

// Test power function
echo "Power function values:", PHP_EOL;
foreach ($numbers as $base) {
    $exponent = 2.0;
    $native_result = pow_native($base, $exponent);
    $php_result = pow_php($base, $exponent);
    $diff = abs($native_result - $php_result);
    echo "  $base^$exponent = $native_result (native), $php_result (PHP), diff: $diff", PHP_EOL;
}

echo PHP_EOL;

// Performance test
echo "Performance comparison (100,000 iterations):", PHP_EOL;

$iterations = 100000;
$test_value = 1.5;

// Test native cosine
$start = microtime(true);
for ($i = 0; $i < $iterations; $i++) {
    cos_native($test_value);
}
$native_time = microtime(true) - $start;

// Test PHP cosine
$start = microtime(true);
for ($i = 0; $i < $iterations; $i++) {
    cos_php($test_value);
}
$php_time = microtime(true) - $start;

echo "  Native cos: " . number_format($native_time * 1000, 3) . " ms", PHP_EOL;
echo "  PHP cos: " . number_format($php_time * 1000, 3) . " ms", PHP_EOL;
echo "  Speedup: " . number_format($php_time / $native_time, 2) . "x", PHP_EOL;
