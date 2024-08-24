// Write a function that takes a string as input and returns the count of vowels in that string. Consider 'a', 'e', 'i', 'o' ,'u' as vowels (both lowercase and uppercase).


fn main() {
    let text1 = "Helloo world";
    let text2 = "ThE quIck brOwn fOx";
    let text3 = "Brrrp";
    println!("{}", count_vowels(text1));
    println!("{}", count_vowels(text2));
    println!("{}", count_vowels(text3));
}

fn count_vowels(input: &str) -> usize {
    let vowels = "aeiouAEIOU";
    input.chars().filter(|c| vowels.contains(*c)).count()
}
