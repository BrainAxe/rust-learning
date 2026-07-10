// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io::{self, Write};

fn calculate_median(nums: &[i32]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }
    let n = nums.len();
    if n % 2 == 1 {
        Some(nums[n / 2] as f64)
    } else {
        Some((nums[n / 2 - 1] as f64 + nums[n / 2] as f64) / 2.0)
    }
}

fn calculate_mode(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let mut map: HashMap<i32, usize> = HashMap::new();
    for num in nums {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    }
    // println!("{map:?}");

    let mut mode = 0;
    let mut max_occurrence = 0;
    for (key, value) in &map {
        // A value becomes the new mode if it either occurs more often than the
        // current best, OR ties the best count but is a smaller number. The tie
        // rule makes the result deterministic (HashMap iteration order is random).
        if *value > max_occurrence || (*value == max_occurrence && *key < mode) {
            max_occurrence = *value;
            mode = *key;
        }
    }
    Some(mode)
}

fn main() {
    let mut input = String::new();
    print!("Enter numbers separated by comma: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut nums: Vec<i32> = input
        .split(',')
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect();

    nums.sort();

    for num in &nums {
        print!("{num}, ");
    }
    println!();

    match calculate_median(&nums) {
        Some(value) => println!("The median is: {value}"),
        None => println!("No median: the list is empty."),
    }

    match calculate_mode(&nums) {
        Some(value) => println!("The mode is: {value}"),
        None => println!("No mode: the list is empty"),
    }
}
