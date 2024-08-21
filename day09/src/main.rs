/* Write a function arrays_are_equal that takes two arrays 
arr1 and arr2 as input and returns true if the arrays are
equal (i.e., contain the same elements in the same order),
and false otherwise. */

fn main() {
    println!("{}",arrays_are_equal(&[1,2,3], &[1,2,3]));
    println!("{}",arrays_are_equal(&[1,2,3], &[1,2,4]));
    println!("{}",arrays_are_equal(&[], &[]));
}

fn arrays_are_equal(arr1 : &[i32], arr2: &[i32]) -> bool {
    if arr1.len() != arr2.len() {
        return false;
    }
    arr1.iter().enumerate().all(|(index, &num)| num == arr2[index])


    /* More idiomatic and efficient
    arr1.len() == arr2.len() && arr1.iter().zip(arr2.iter()).all(|(&a, &b)| a == b ) */
}
