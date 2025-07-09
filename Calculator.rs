// File: Calculator.rs
// Author: Ben Bos
// Date: 2025-07-08

// Description:
// This is a simple calculator program in Rust that performs basic arithmetic operations.
// It takes two numbers and an operation from the user, performs the operation, and prints the result.
// It includes functions for addition, subtraction, multiplication, and division.
// It also handles user input and output, ensuring that the program is interactive.
// The program uses the standard input/output library for reading user input and printing results.
// The code is structured to be clear and easy to understand, with separate functions for each operation.
// The program is designed to be run in a Rust environment and will prompt the user for input
// before performing calculations. It includes error handling for invalid number inputs and operations.
// The program is a good example of basic Rust syntax and functionality, demonstrating how to work with
// user input, perform arithmetic operations, and handle errors gracefully.
// This code is a complete, functional Rust program that can be compiled and run to perform basic
// arithmetic calculations based on user input.
// It is a good starting point for learning Rust and understanding how to build simple command-line applications
// that interact with users.
// The program is structured to be modular, with separate functions for each arithmetic operation,
// making it easy to extend or modify in the future if additional operations are needed.
// The use of `match` for operation selection allows for clear and concise handling of different operations
// based on user input, providing a good example of Rust's powerful pattern matching capabilities.
// The program is designed to be user-friendly, prompting the user for input and providing clear output
// that indicates the result of the calculation.
// The code is well-commented, making it easy to follow and understand the flow of the program.
// It is a practical example of how to implement a simple calculator in Rust, showcasing the language's
// capabilities for handling user input, performing arithmetic operations, and managing program flow.

// Imports
use std::io::{self, Write};
use std::process;

// Function definitions for arithmetic operations

// Addition
fn addition(input1: f64, input2: f64) -> f64
{
    return input1 + input2;
}

// Subtraction
fn subtraction(input1: f64, input2: f64) -> f64
{
    return input1 - input2;
}

// Multiplication
fn multiplication(input1: f64, input2: f64) -> f64
{
    return input1 * input2;
}

// Division
// Note: This function does not handle division by zero; it assumes valid input.
fn division(input1: f64, input2: f64) -> f64
{
    // Check for division by zero
    if input2 == 0.0 {
        println!("Error: Division by zero is not allowed.");
        process::exit(1); // Exit the program with an error code
    }

    // Perform division
    return input1 / input2;
}

// Modulus
fn modulus(input1: f64, input2: f64) -> f64
{
    return input1 % input2;
}

// Main function to run the calculator
fn main() {
    // Variables
    let mut operand1 = String::new();
    let mut operand2 = String::new();
    let mut operator = String::new();

    // Prompt user for first operand
    // Force flushing the output buffer to ensure prompt appears before input
    print!("Enter First number: ");
    io::stdout().flush().unwrap();

    // Capture user input for the first operand
    io::stdin().read_line(&mut operand1).expect("Failed to read line");

    // Remove any leading/trailing whitespace and parse the input
    // Expecting a valid integer input
    // If parsing fails, the program will panic with an error message
    let cleaned_operand1 = operand1.trim();
    let parsed_operand1: f64 = cleaned_operand1.parse().expect("Invalid number input");
    
    // Prompt user for second operand
    // Force flushing the output buffer to ensure prompt appears before input
    print!("Enter Second number: ");
    io::stdout().flush().unwrap();

    // Capture user input for the second operand
    io::stdin().read_line(&mut operand2).expect("Failed to read line");

    // Remove any leading/trailing whitespace and parse the input
    // Expecting a valid integer input
    // If parsing fails, the program will panic with an error message
    let cleaned_operand2 = operand2.trim();
    let parsed_operand2: f64 = cleaned_operand2.parse().expect("Invalid number input");
    
    // Prompt user for operator
    // Force flushing the output buffer to ensure prompt appears before input
    print!("What operation you would like to do: ");
    io::stdout().flush().unwrap();

    // Capture user input for the operator
    io::stdin().read_line(&mut operator).expect("Failed to read line");

    // Remove any leading/trailing whitespace and parse the input
    let cleaned_operator: String = String::from(operator.trim());
    
    // Match the cleaned operator to perform the corresponding arithmetic operation
    // If the operator is not recognized, it defaults to printing an error message and returning
    let return_val: f64 = match cleaned_operator.as_str()
    {
        "+" => addition(parsed_operand1, parsed_operand2),
        "-" => subtraction(parsed_operand1, parsed_operand2),
        "*" => multiplication(parsed_operand1, parsed_operand2),
        "/" => division(parsed_operand1, parsed_operand2),
        "%" => modulus(parsed_operand1, parsed_operand2),
        _ => 
        {
            println!("Invalid operation!");
            0.0
        },
    };
    
    // Print the result of the operation
    // The result is formatted to two decimal places for better readability
    println!("{0} {1} {2} = {3:.2}", parsed_operand1, cleaned_operator, parsed_operand2, return_val);
}