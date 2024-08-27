/*
Validate Parentheses

You are given a string s consisting of the following characters: '(', ')', '{', '}', '[' and ']'.

The input string s is valid if and only if:

    Every open bracket is closed by the same type of close bracket.
    Open brackets are closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.

Return true if s is a valid string, and false otherwise.A
*/

fn main() {
    println!("{}",is_valid("[]"));
    println!("{}", is_valid("([{}])"));
    println!("{}", is_valid("[(])"));
}

fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => match stack.pop() { 
                Some('(') => {}, 
                _ => return false, 
            },
            '}' => match stack.pop() { 
                Some('{') => {},
                _ => return false, 
            },
            ']' => match stack.pop() { 
                Some('[') => {}, 
                _ => return false,
            },
            _ => return false,
        }
    }

    stack.is_empty()
}
