use std::io;

fn main() {
    println!("Simple Calculator");

    // Input for first number
    let num1: f64 = input_number("Enter the first number:");

    // Input for operator
    let operator: char = input_operator("Enter an operator (+, -, *, /):");

    // Input for second number
    let num2: f64 = input_number("Enter the second number:");

    // Perform the calculation based on the operator
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Error: Division by zero");
                std::f64::NAN // Return NaN for division by zero
            } else {
                num1 / num2
            }
        },
        _ => {
            println!("Error: Invalid operator");
            std::f64::NAN // Return NaN for invalid operator
        }
    };

    // Display the result
    println!("Result: {}", result);
}

// Function to get a valid number input
fn input_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
}

// Function to get a valid operator input
fn input_operator(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let operator = input.trim().chars().next();
        match operator {
            Some(op) if op == '+' || op == '-' || op == '*' || op == '/' => break op,
            _ => {
                println!("Invalid operator. Please enter one of: +, -, *, /");
                continue;
            }
        }
    }
}
