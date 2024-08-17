/*Write a function find_max that takes an array of numbers
as input and returns the maximum number in the array.*/ 


fn main() {
    let numbers1 = vec![1, 5, 3, 9, 2];
    let numbers2 = vec![-10, -5, -3, -9, -2];
    let numbers3 = vec![5];
    println!("{}", find_max(numbers1));
    println!("{}", find_max(numbers2));
    println!("{}", find_max(numbers3));
}

fn find_max(vector: Vec<i32>) -> i32 {
    let max: i32 = vector.iter().fold(vector[0], |acc, &x| if acc >= x {acc} else {x} );
    max
}
