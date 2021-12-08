// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let mut sum: i32 = 0;
//     for line in lines {
//         let splitted: Vec<&str> = line.split(" | ").collect();
//         let right = splitted[1];
//         let words: Vec<&str> = right.split(" ").collect();

//         for word in words {
//             let len = word.chars().count();
//             if len == 2 || len == 3 || len == 4 || len == 7 {
//                 sum = sum + 1;
//             }
//         }
//     }

//     println!("{}", sum);
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut sum: i32 = 0;
    for line in lines {
        let splitted: Vec<&str> = line.split(" | ").collect();
        let left = splitted[0];
        let right = splitted[1];
        let left_words : Vec<&str> = left.split(" ").collect();
        let words: Vec<&str> = right.split(" ").collect();
        let mut digits = ['\0', '\0', '\0' ,'\0', '\0', '\0', '\0'];

        let mut one: &str = "\0";
        let mut four: &str = "\0";
        let mut seven: &str = "\0";
        let mut eight: &str = "\0";
        let mut four_five: Vec<i32> = vec![0; 8];
        for word in &left_words {
            let len = word.chars().count();
            if len == 2 {
                one = word;
            } else if len == 3 {
                seven = word;
            } else if len == 4 {
                four = word;
                for ch in word.chars() {
                    four_five[ch as usize - 97] += 1;
                }
            } else if len == 5 {
                for ch in word.chars() {
                    four_five[ch as usize - 97] += 1;
                }
            } else if len == 7 {
                eight = word;
            }
        }
        let mut first_cnt = 0;
        for word in left_words {
            let len = word.chars().count();
            if len == 6 {
                for ch in word.chars() {
                    if ch == one.chars().nth(0).unwrap() {
                        first_cnt += 1;
                    }
                }
            }
        }
        if first_cnt == 2 {
            digits[0] = one.chars().nth(0).unwrap();
            digits[1] = one.chars().nth(1).unwrap();
        } else if first_cnt == 3 {
            digits[1] = one.chars().nth(0).unwrap();
            digits[0] = one.chars().nth(1).unwrap();
        }
        for ch in seven.chars() {
            if ch != digits[0] && ch != digits[1] {
                digits[4] = ch;
            }
        }
        for i in 0..7 {
            let ch = std::char::from_u32((98 + i - 1) as u32).unwrap();
            if four_five[i] == 4 {
                digits[5] = ch;
            }
            if four_five[i] == 3 && ch != digits[0] && ch != digits[1] && ch != digits[4] {
                digits[6] = ch;
            }
        }
        for ch in four.chars() {
            if ch != digits[5] && ch != digits[1]  && ch != digits[0] {
                digits[2] = ch;
            }
        }
        for ch in eight.chars() {
            if ch != digits[0] && ch != digits[1] && ch != digits[2] && ch != digits[4] && ch != digits[5] && ch != digits[6] {
                digits[3] = ch;
            }
        }
        let mut one_num: i32 = 0;
        for word in words {
            let mut num_vec: Vec<i32> = Vec::new();
            for ch in word.chars() {
                for i in 0..7 {
                    if ch == digits[i] {
                        num_vec.push(i as i32);
                    }
                }
            }
            let zero: Vec<i32> = [0, 1, 2, 3, 4, 6].to_vec();
            let one: Vec<i32> = [0, 1].to_vec();
            let two: Vec<i32> = [4, 0 , 5, 3, 6].to_vec();
            let three: Vec<i32> = [4, 0 ,1, 5, 6].to_vec();
            let four: Vec<i32> = [2, 5, 0, 1].to_vec();
            let five: Vec<i32> = [4, 2, 5, 1, 6].to_vec();
            let six: Vec<i32> = [4, 2, 5, 1, 3, 6].to_vec();
            let seven: Vec<i32> = [4, 0, 1].to_vec();
            let eight: Vec<i32> = [0, 1, 2, 3, 4, 5, 6].to_vec();
            let nine: Vec<i32> = [0, 1, 2, 4, 5, 6].to_vec();
            if num_vec.len() == zero.len() && num_vec.iter().all(|p| zero.contains(p)) {
                one_num = one_num * 10 + 0;
            }
            if num_vec.len() == one.len() && num_vec.iter().all(|p| one.contains(p)) {
                one_num = one_num * 10 + 1;
            }
            if num_vec.len() == two.len() && num_vec.iter().all(|p| two.contains(p)) {
                one_num = one_num * 10 + 2;
            }
            if num_vec.len() == three.len() && num_vec.iter().all(|p| three.contains(p)) {
                one_num = one_num * 10 + 3;
            }
            if num_vec.len() == four.len() && num_vec.iter().all(|p| four.contains(p)) {
                one_num = one_num * 10 + 4;
            }
            if num_vec.len() == five.len() && num_vec.iter().all(|p| five.contains(p)) {
                one_num = one_num * 10 + 5;
            }
            if num_vec.len() == six.len() && num_vec.iter().all(|p| six.contains(p)) {
                one_num = one_num * 10 + 6;
            }
            if num_vec.len() == seven.len() && num_vec.iter().all(|p| seven.contains(p)) {
                one_num = one_num * 10 + 7;
            }
            if num_vec.len() == eight.len() && num_vec.iter().all(|p| eight.contains(p)) {
                one_num = one_num * 10 + 8;
            }
            if num_vec.len() == nine.len() && num_vec.iter().all(|p| nine.contains(p)) {
                one_num = one_num * 10 + 9;
            }
        }
        sum += one_num;
    }

    println!("{}", sum);
}