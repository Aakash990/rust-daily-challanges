/* Pairwise

Given an array arr, find element pairs whose sum equal the second argument arg and return the sum of their indices.

You may use multiple pairs that have the same numeric elements but different indices. Each pair should use the lowest possible available indices. Once an element has been used it cannot be reused to pair with another element. For instance, pairwise([1, 1, 2], 3) creates a pair [2, 1] using the 1 at index 0 rather than the 1 at index 1, because 0+2 < 1+2.

For example pairwise([7, 9, 11, 13, 15], 20) returns 6. The pairs that sum to 20 are [7, 13] and [9, 11]. We can then write out the array with their indices and values.
Index 	0 	1 	2 	3 	4
Value 	7 	9 	11 	13 	15

Below we'll take their corresponding indices and add them.

7 + 13 = 20 → Indices 0 + 3 = 3
9 + 11 = 20 → Indices 1 + 2 = 3
3 + 3 = 6 → Return 6
*/

fn pairwise(arr: Vec<i32>, arg: i32) -> i32{
    let mut sum = 0;
    let mut used = vec![false; arr.len()];

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if !used[i] && !used[j] && arr[i] + arr[j] == arg {
                sum += (i + j) as i32;
                used[i] = true;
                used[j] = true;
                break;
            }
        }
    }
    sum
}


fn main() {
    println!("{}", pairwise(vec![1, 4, 2, 3, 0, 5], 7));  // should return 11
    println!("{}", pairwise(vec![1, 3, 2, 4], 4));        // should return 1
    println!("{}", pairwise(vec![1, 1, 1], 2));           // should return 1
    println!("{}", pairwise(vec![0, 0, 0, 0, 1, 1], 1));  // should return 10
    println!("{}", pairwise(vec![], 100));                // should return 0
}
