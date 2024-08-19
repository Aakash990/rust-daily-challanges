/* Write a function factorial that takes a non-negative integer num as input and returns its factorial. The factorial of a non-negative integer n, denoted as n!, is the product of all positive integers less than or equal to n. The factorial of 0 is defined as 1.*/

fn main() {
    println!("{}",factorial(3));
    println!("{}", factorial(0));
    println!("{}", factorial(5));
}

/// Compute the factorial of a non-negative integer.
fn factorial(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => num * factorial(num - 1),
    }
}
