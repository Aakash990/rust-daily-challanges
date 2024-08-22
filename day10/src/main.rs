/* Write a function that takes a number as input and returns the sum of its digits. */


fn main() {
    println!("{}", sum_of_digits(1234));
    println!("{}", sum_of_digits(4321));
    println!("{}", sum_of_digits(123456));
}

fn sum_of_digits(num: u64) -> u64 {
    let sum = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum();
    sum
}
