/* Write a function called countChar that takes two parameters: a string and a character to count. 
The function should return the number of times the specified character appears in 
the string. */


fn main() {
    let word: String = String::from("Assassination");
    let result = count_char(word, 's');
    println!("{}", result);
}

fn count_char(word: String, character: char) -> u8 {
    let mut count: u8 = 0;
    let upper_character = character.to_uppercase().next().unwrap();
    for single_char in word.chars() {
        for upper_char in single_char.to_uppercase() {
            if upper_char == upper_character {
                count += 1;
            }
        }
    }
    count
}
