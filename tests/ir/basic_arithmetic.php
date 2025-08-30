<?php
/**
 * Test basic arithmetic operations
 */

function add(int $a, int $b): int {
    return $a + $b;
}

function subtract(int $a, int $b): int {
    return $a - $b;
}

function multiply(int $a, int $b): int {
    return $a * $b;
}

function divide(int $a, int $b): int {
    return $a / $b;
}

function modulo(int $a, int $b): int {
    return $a % $b;
}

function power(int $a, int $b): int {
    return $a ** $b;
}

// Test cases
$result1 = add(10, 5);      // 15
$result2 = subtract(10, 5);  // 5
$result3 = multiply(10, 5);  // 50
$result4 = divide(10, 5);    // 2
$result5 = modulo(10, 3);    // 1
$result6 = power(2, 3);      // 8

echo "Basic arithmetic test results:", PHP_EOL;
echo "10 + 5 = $result1", PHP_EOL;
echo "10 - 5 = $result2", PHP_EOL;
echo "10 * 5 = $result3", PHP_EOL;
echo "10 / 5 = $result4", PHP_EOL;
echo "10 % 3 = $result5", PHP_EOL;
echo "2 ^ 3 = $result6", PHP_EOL;
