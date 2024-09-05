/* No Repeats Please

Return the number of total permutations of the provided string that don't have repeated consecutive letters. Assume that all characters in the provided string are each unique.

For example, aab should return 2 because it has 6 total permutations (aab, aab, aba, aba, baa, baa), but only 2 of them (aba and aba) don't have the same letter (in this case a) repeating.
*/

fn get_permutations(s: &mut Vec<char>, start: usize, result: &mut Vec<Vec<char>>) {
    if start == s.len() {
        result.push(s.clone());
    } else {
        for i in start..s.len() {
            s.swap(start, i);
            get_permutations(s, start + 1, result);
            s.swap(start, i); //backtrack
        }
    }
}

fn has_no_repeats(perm: &Vec<char>) -> bool {
    for i in 0..perm.len()-1 {
        if perm[i] == perm[i + 1] {
            return false;
        }
    }
    true
}

fn perm_alone(s: &str) -> usize{
    let mut chars: Vec<char> = s.chars().collect();
    let mut permutations = vec![];

    // Generating all permutations
    get_permutations(&mut chars, 0, &mut permutations);

    
    // Filtering permutations to include only those with no consecutive repeats
    let valid_permutations: Vec<Vec<char>> = permutations
        .into_iter()
        .filter(|perm| has_no_repeats(perm))
        .collect();
    
    // Returning the number of valid permutations
    valid_permutations.len()

}

fn main() {
    println!("{}", perm_alone("aab"));
    println!("{}", perm_alone("aaa"));
    println!("{}", perm_alone("aabb"));
    println!("{}", perm_alone("abcdefa"));
    println!("{}", perm_alone("zzzzzzz"));
    println!("{}", perm_alone("a"));
    println!("{}", perm_alone("aaab"));
    println!("{}", perm_alone("aaabb")); 
}
