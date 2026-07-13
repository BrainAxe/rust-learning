fn main() {
    let str1 = String::from("Lifetime");
    let str2 = String::from("Lifetimes");
    let result = longest(&str1, &str2);
    println!("The Longest String is -> {result}");

    // let str1 = String::from("Lifetime");
    // let result: &str;
    // {
    //     let str2 = String::from("Lifetimes");
    //     result = longest(&str1, &str2);
    // }
    // println!("The Longest String is -> {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
