<?php
/**
 * Object-Oriented Programming example for php2ir compiler
 * 
 * This demonstrates OOP features that the compiler supports:
 * - Class definitions
 * - Properties and methods
 * - Constructor
 * - Inheritance
 * - Method calls
 */

class Person {
    private string $name;
    private int $age;
    
    public function __construct(string $name, int $age) {
        $this->name = $name;
        $this->age = $age;
    }
    
    public function getName(): string {
        return $this->name;
    }
    
    public function getAge(): int {
        return $this->age;
    }
    
    public function isAdult(): bool {
        return $this->age >= 18;
    }
    
    public function greet(): string {
        return "Hello, I'm {$this->name} and I'm {$this->age} years old.";
    }
}

class Student extends Person {
    private string $school;
    
    public function __construct(string $name, int $age, string $school) {
        parent::__construct($name, $age);
        $this->school = $school;
    }
    
    public function getSchool(): string {
        return $this->school;
    }
    
    public function greet(): string {
        $parent_greeting = parent::greet();
        return $parent_greeting . " I study at {$this->school}.";
    }
}

// Create instances
$person = new Person("Alice", 25);
$student = new Student("Bob", 16, "High School");

// Demonstrate method calls
echo $person->greet(), PHP_EOL;
echo $student->greet(), PHP_EOL;

// Demonstrate inheritance
echo "Is Alice an adult? " . ($person->isAdult() ? "Yes" : "No"), PHP_EOL;
echo "Is Bob an adult? " . ($student->isAdult() ? "Yes" : "No"), PHP_EOL;

// Demonstrate property access
echo "Bob's school: " . $student->getSchool(), PHP_EOL;
