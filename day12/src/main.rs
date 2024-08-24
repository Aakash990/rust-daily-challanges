/* Write a function that takes an array of integers as input and removes 
any duplicate elements, returning a new array with only the unique elements. */

use std::collections::HashSet;

fn main() {
    println!("{:?}", remove_duplicates(&[1,2,3,2,1,4]));
    println!("{:?}", remove_duplicates(&[5,6,7,7,8,8,9])); 
    println!("{:?}", remove_duplicates(&[1,2,3,4]));
    println!("{:?}", remove_duplicates(&[]));
}

fn remove_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut unique_elements = HashSet::new();
    arr.iter()
        .filter(|&x| unique_elements.insert(*x))
        .cloned()
        .collect()
}
