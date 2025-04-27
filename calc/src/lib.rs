// build calculator functions

// add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// subtract two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// multiply two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// divide two numbers
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
