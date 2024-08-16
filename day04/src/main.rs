/*Write a Rust function called is_palindrome that determines whether a given string is a 
palindrome, ignoring spaces and commas, and handling case insensitivity.*/

fn main() {
    println!("{}",is_palindrome("Hello world"));
    println!("{}",is_palindrome("A man, a plan, a canal, Panama"));
}

fn is_palindrome( s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| !c.is_whitespace() && *c != ',')
        .collect::<String>()
        .to_lowercase();
    let reversed: String = cleaned.chars().rev().collect();
    if cleaned == reversed {
        true
    } else {
        false
    }
}
