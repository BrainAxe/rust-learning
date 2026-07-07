// Reverse words

fn reverse_words(sentence: &str) -> String {
    let mut reverse_output = String::new();

    for word in sentence.split(' ').rev() {
        // println!("{word}");
        if !reverse_output.is_empty() {
            reverse_output.push(' ');
        }
        reverse_output.push_str(word);
    }
    reverse_output
}

fn main() {
    let my_sentence = "hello world foo";
    let output = reverse_words(my_sentence);
    println!("{output}");
}
