//Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn convert_to_pig_latin(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }
    let first_char = word.chars().next().unwrap();
    if first_char.is_alphabetic() {
        if is_vowel(first_char) {
            format!("{word}-hay")
        } else {
            let rest_of_word = word.chars().skip(1).collect::<String>();
            format!("{rest_of_word}-{first_char}ay")
        }
    } else {
        word.to_string()
    }
}

fn main() {
    let input_str = "first";
    // let input_str = "apple";
    // let input_str = "über";
    // let input_str = "";
    // let input_str = "123";
    let output_str = convert_to_pig_latin(input_str);
    println!("{input_str} -> {output_str}");
}
