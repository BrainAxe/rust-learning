// Find the largest number

fn largest<T: std::cmp::PartialOrd>(numbers: &[T]) -> &T {
    let mut largest_num = &numbers[0];

    for num in numbers {
        if num > largest_num {
            largest_num = num;
        }
    }
    largest_num
}

fn main() {
    let nums = vec![25, 11, 45, 37, 105, 97, 88];
    let result = largest(&nums);
    println!("The Largest Number is: {result}");

    let fractions = vec![25.0, 11.0, 37.5, 88.8];
    let result = largest(&fractions);
    println!("The Largest Number is: {result}");

    let chars = vec!['r', 't', 'y', 'z'];
    let result = largest(&chars);
    println!("The Largest char is: {result}");
}
