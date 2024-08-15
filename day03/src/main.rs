/*Write a function to sort an array of numbers in an ascending order.*/

fn main() {
     let numbers: [i32; 5] = [10,34,3,25,9];
     println!("{:?}", sort_array(numbers));
}

fn sort_array(mut numbers: [i32; 5]) -> [i32; 5] {
    numbers.sort();
    numbers
}
