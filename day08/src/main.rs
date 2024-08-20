/* Write a function called calculate_average that takes an 
array of numbers as input and returns the average of those
numbers.*/


fn main() {
    println!("{}",calculate_average(&[5,10,2,8]));
}

fn calculate_average(vector: &[i32]) -> f32 {
    let sum: i32 = vector.iter().sum();
    sum as f32 / vector.len() as f32
}
