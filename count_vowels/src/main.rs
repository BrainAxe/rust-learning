// Count VOWELS

fn count_vowels(s: &str) -> u32 {
    let mut count = 0;
    for i in s.to_ascii_lowercase().chars() {
        // println!("{i}");
        if "aeiou".contains(i) {
            count += 1;
        }
    }
    count
}

fn main() {
    let my_str = "Hello, there!";
    let result = count_vowels(my_str);
    println!("{my_str} -> {result}");
}
