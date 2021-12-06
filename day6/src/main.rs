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

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let splited_line: Vec<&str> = content.split(",").collect();
    let mut nums: Vec<i128> = vec![0; 9];

    for each in splited_line {
        let num: usize = each.parse().unwrap();
        nums[num] = nums[num] + 1;
    }

    for _i in 0..256 {
        let zero_count = nums[0];
        nums.rotate_left(1);
        nums[6] = nums[6] + zero_count;
    }

    let mut sum = 0;
    for i in 0..9 {
        sum = sum + nums[i];
    }

    println!("{}", sum);
}