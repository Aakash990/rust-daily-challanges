/*Write a function longest_word that return a longest word from the sentence. */


fn main() {
    let sentence: String = String::from("This is a great way to learn rust programming language");
    match longest_word(sentence) {
        Some(word) => println!("The longest word is: {}", word),
        None => println!("No words found"),
    }
}

fn longest_word(s: String) -> Option<String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.iter()
        .max_by_key(|&word| word.len())
        .map(|&word| word.to_string())  
}
