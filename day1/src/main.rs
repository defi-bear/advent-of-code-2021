// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let numbers = content.split("\n");

//     let mut prev = 0;
//     let mut count = 0;
//     for number in numbers {
//         let num: i32 = number.parse().unwrap();
//         if num > prev {
//             count = count + 1;
//         }
//         prev = num;
//     }

//     println!("{}", count - 1);
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("something went wrong");
    let numbers: Vec<&str> = content.split("\n").collect();

    let mut prev = 0;
    let mut count = 0;
    for (pos, e) in numbers.iter().enumerate() {
        if pos + 2 == numbers.len() {
            break;
        }
        let first: i32 = e.parse().unwrap();
        let second: i32 = numbers[pos + 1].parse().unwrap();
        let third: i32 = numbers[pos + 2].parse().unwrap();
        let sum: i32 = first + second + third;
        println!("{}", sum);
        if sum > prev {
            count = count + 1;
        }
        prev = sum;
    }

    println!("{}", count - 1);
}
