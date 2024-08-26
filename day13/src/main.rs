/* Write a function called is_power_of_two that takes an integer num as 
input and returns true if num is a power of two, and false otherwise. */

fn main() {
    println!("{}", is_power_of_two(8));
    println!("{}", is_power_of_two(7));
}

fn is_power_of_two(num: u32) -> bool {  
    num > 0 && (num & (num -1) == 0)
}
