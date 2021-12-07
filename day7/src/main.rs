// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let splited_line: Vec<&str> = content.split(",").collect();
//     let mut nums: Vec<i32> = Vec::new();

//     for each in splited_line {
//         nums.push(each.parse().unwrap());
//     }

//     let max = &nums.iter().max().unwrap().clone();
//     let min = &nums.iter().min().unwrap().clone();
//     let mut min_val: i32 = min.clone();
//     let mut min_sum: i32 = nums.iter().sum();
//     println!("min_val: {}, min_sum: {}", min_val, min_sum);
//     for i in min.clone()..max.clone() {
//         let mut sum: i32 = 0;
//         for num in &nums {
//             let sub = num - i;
//             sum = sum + i32::abs(sub);
//         }
//         if sum < min_sum {
//             min_sum = sum;
//             min_val = i.clone();
//         }
//     }
 
//     println!("{}, {}", min_val, min_sum);
// }
use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let splited_line: Vec<&str> = content.split(",").collect();
    let mut nums: Vec<i32> = Vec::new();

    for each in splited_line {
        nums.push(each.parse().unwrap());
    }

    let max = &nums.iter().max().unwrap().clone();
    let min = &nums.iter().min().unwrap().clone();
    let mut min_sum: i32 = nums.iter().sum();

    for i in min.clone()..max.clone() {
        let mut sum: i32 = 0;
        for num in &nums {
            let sub = i32::abs(num-i)+1;
            sum = sum + (sub - 1) * sub / 2;
        }
        if i == min.clone() {
            min_sum = sum;
        }
        else if sum < min_sum {
            min_sum = sum;
        }
    }
 
    println!("{}", min_sum);
}