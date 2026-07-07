// Find the largest number in the array

fn largest(numbers: &[i32]) -> i32 {
    let mut largest_num = numbers[0];

    // Dereference
    // for num in numbers {
    //     if *num >= largest_num {
    //         largest_num = *num;
    //     }
    // }

    // Destructure
    for &num in numbers {
        if num >= largest_num {
            largest_num = num;
        }
    }
    largest_num
}

fn main() {
    let nums = [25, 11, 45, 37, 105, 97, 88];
    let result = largest(&nums);

    println!("The Largest Number is: {result}");
}
