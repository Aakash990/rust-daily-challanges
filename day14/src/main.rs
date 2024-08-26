/* Write a function to calculate the sum of squares of all elements in an array. For
example, given the array [1, 2, 3], the function should return 14 because 1*1 + 2*2 + 3*3 = 1 + 4 + 9 =14 */


fn main() {
    println!("{}", sum_of_squares(&[1, 2, 3]));
}

fn sum_of_squares(arr : &[i32]) -> i32 {
    arr.iter().map(|&num| num * num ).sum()
}
