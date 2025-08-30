<?php
/**
 * Test control flow constructs
 */

function test_if_else(int $value): string {
    if ($value > 0) {
        return "positive";
    } elseif ($value < 0) {
        return "negative";
    } else {
        return "zero";
    }
}

function test_switch(int $value): string {
    switch ($value) {
        case 1:
            return "one";
        case 2:
            return "two";
        case 3:
            return "three";
        default:
            return "other";
    }
}

function test_while_loop(int $max): array {
    $result = [];
    $i = 0;
    while ($i < $max) {
        $result[] = $i;
        $i++;
    }
    return $result;
}

function test_for_loop(int $max): array {
    $result = [];
    for ($i = 0; $i < $max; $i++) {
        $result[] = $i * 2;
    }
    return $result;
}

function test_foreach_loop(array $items): array {
    $result = [];
    foreach ($items as $item) {
        $result[] = $item * 2;
    }
    return $result;
}

function test_match(int $value): string {
    return match($value) {
        1 => "one",
        2 => "two",
        3 => "three",
        default => "other"
    };
}

// Test cases
echo "Control flow test results:", PHP_EOL;

// Test if-else
echo "test_if_else(5): " . test_if_else(5), PHP_EOL;
echo "test_if_else(-3): " . test_if_else(-3), PHP_EOL;
echo "test_if_else(0): " . test_if_else(0), PHP_EOL;

// Test switch
echo "test_switch(2): " . test_switch(2), PHP_EOL;
echo "test_switch(5): " . test_switch(5), PHP_EOL;

// Test while loop
$while_result = test_while_loop(5);
echo "test_while_loop(5): [" . implode(", ", $while_result) . "]", PHP_EOL;

// Test for loop
$for_result = test_for_loop(5);
echo "test_for_loop(5): [" . implode(", ", $for_result) . "]", PHP_EOL;

// Test foreach loop
$foreach_result = test_foreach_loop([1, 2, 3, 4, 5]);
echo "test_foreach_loop([1,2,3,4,5]): [" . implode(", ", $foreach_result) . "]", PHP_EOL;

// Test match
echo "test_match(3): " . test_match(3), PHP_EOL;
echo "test_match(7): " . test_match(7), PHP_EOL;
