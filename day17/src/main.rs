/* The mathematical term symmetric difference (△ or ⊕) of two sets is the set of elements which are in either of the two sets but not in both. For example, for sets A = {1, 2, 3} and B = {2, 3, 4}, A △ B = {1, 4}.

Symmetric difference is a binary operation, which means it operates on only two elements. So to evaluate an expression involving symmetric differences among three elements (A △ B △ C), you must complete one operation at a time. Thus, for sets A and B above, and C = {2, 3}, A △ B △ C = (A △ B) △ C = {1, 4} △ {2, 3} = {1, 2, 3, 4}.

Create a function that takes two or more arrays and returns an array of their symmetric difference. The returned array must contain only unique values (no duplicates). */

use std::collections::HashSet;

// Helper function to compute the symmetric difference of two sets
fn sym_diff(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    let mut result = HashSet::new();
   
    // adding elements from set1 that are not in set2 
    for &item in set1 {
        if !set2.contains(&item) {
            result.insert(item);
        }
    }

    // adding elements from set2 that are not in set1
    for &item in set2 {
        if !set1.contains(&item) {
            result.insert(item);
        }
    }

    result
}

// main function to compute a symmetric difference of multiple arrays
fn sym(args: Vec<Vec<i32>>) -> Vec<i32> {
    // converting the first array to HashSet
    let mut result_set: HashSet<i32> = HashSet::from_iter(args[0].iter().cloned());

    // computing symmetric difference iteratively for all arrays
    for arr in args.iter().skip(1) {
        let current_set: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());
        result_set = sym_diff(&result_set, &current_set);
    }
    
    // converting the result set to a sorted vector and return it
    let mut result_vec: Vec<i32> = result_set.into_iter().collect();
    result_vec.sort();
    result_vec
}

fn main() {
    // Testing the function
    println!("{:?}", sym(vec![vec![1, 2, 3], vec![5, 2, 1, 4]]));
    println!("{:?}", sym(vec![vec![1, 2, 3, 3], vec![5, 2, 1, 4]]));
    println!("{:?}", sym(vec![vec![1, 2, 5], vec![2, 3, 5], vec![3, 4, 5]]));
}
