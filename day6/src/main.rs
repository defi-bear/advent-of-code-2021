// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let splited_line: Vec<&str> = content.split(",").collect();
//     let mut nums: Vec<i32> = Vec::new();

//     for each in splited_line {
//         nums.push(each.parse().unwrap());
//     }

//     for _i in 0..80 {
//         let mut new_count: usize = 0;
//         for j in 0..nums.len() {
//             if nums[j] == 0 {
//                 nums[j] = 6;
//                 new_count = new_count + 1;
//             } else {
//                 nums[j] = nums[j] - 1;
//             }
//         }
//         for _j in 0..new_count {
//             nums.push(8);
//         }
//     }

//     println!("{}", nums.len());
// }
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, One}; // 0.2.8

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let splited_line: Vec<&str> = content.split(",").collect();
    let zero: BigUint = Zero::zero();
    let mut nums: Vec<BigUint> = vec![zero; 9];

    for each in splited_line {
        let num: usize = each.parse().unwrap();
        let one: BigUint = One::one();
        nums[num] += one;
    }

    for _i in 0..100000 {
        let zero_count: BigUint = nums[0].clone();
        nums.rotate_left(1);
        nums[6] += zero_count;
    }

    let mut sum: BigUint = Zero::zero();
    for i in 0..9 {
        sum += &nums[i];
    }

    println!("{}", sum);
}