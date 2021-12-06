// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let words: Vec<&str> = lines[0].split(",").collect();
//     let mut numbers : Vec<i32> = Vec::new();

//     for i in 2..lines.len() {
//         let line: Vec<&str> = lines[i].split(" ").collect();
//         if line.len() == 5 {
//             for j in 0..5 {
//                 numbers.push(line[j].parse().unwrap());
//             }
//         }
//     }
//     println!("{}", numbers.len());
//     let mut num_index: Vec<i32> = vec![0; numbers.len()];

//     for i in 0..words.len() {
//         let bingo: i32 = words[i].parse().unwrap();

//         for j in 0..numbers.len() {
//             if bingo == numbers[j] {
//                 num_index[j] = 1;
//             }
//         }
//         let mut row = 0;
//         for j in 0..numbers.len()/5 {
//             if num_index[j * 5] == 1 && num_index[j * 5 + 1] == 1 && num_index[j * 5 + 2] == 1 && num_index[j * 5 + 3] == 1 && num_index[j * 5 + 4] == 1 {
//                 row = (j + (5 - j % 5)) / 5 - 1;
//             }
//         }
//         if row != 0 {
//             let mut sum: i32 = 0;
//             for j in 25 * row..25 * (row + 1) {
//                 if num_index[j] == 0 {
//                     sum = sum + numbers[j];
//                 }
//             }

//             println!("{}", sum * bingo);
//             break;
//         }
//     }
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let words: Vec<&str> = lines[0].split(",").collect();
    let mut numbers : Vec<i32> = Vec::new();

    for i in 2..lines.len() {
        let line: Vec<&str> = lines[i].split(" ").collect();
        if line.len() == 5 {
            for j in 0..5 {
                numbers.push(line[j].parse().unwrap());
            }
        }
    }
    println!("{}", numbers.len());
    let mut num_index: Vec<i32> = vec![0; numbers.len()];
    let mut marked_boards = Vec::new();

    for i in 0..words.len() {
        let bingo: i32 = words[i].parse().unwrap();
        println!("Checking the bingo {}", bingo);

        for j in 0..numbers.len() {
            if bingo == numbers[j] {
                num_index[j] = num_index[j] + 1;
            }
        }
        let mut row = 999;
        for j in 0..numbers.len()/5 {
            let bn = (j + (5 - j % 5)) / 5 - 1;
            let col = j % 5;
            if !marked_boards.contains(&bn) {
                if num_index[j * 5] == 1 && num_index[j * 5 + 1] == 1 && num_index[j * 5 + 2] == 1 && num_index[j * 5 + 3] == 1 && num_index[j * 5 + 4] == 1 {
                    row = bn;
                    println!("{}, {}", bingo, row);
                    
                    if row != 999 {
                        let mut sum: i32 = 0;
                        for j in 25 * row..25 * (row + 1) {
                            if num_index[j] == 0 {
                                sum = sum + numbers[j];
                            }
                        }
                        marked_boards.push(row);
                        println!("{}: {} * {} = {}", marked_boards.len(), sum, bingo, sum * bingo);
                    }
                }
                if num_index[(bn * 5)*5 + col] == 1 && num_index[(bn * 5 +1)*5 + col] == 1 && num_index[(bn * 5 +2)*5 + col] == 1 && num_index[(bn * 5 +3)*5 + col] == 1 && num_index[(bn * 5 +4)*5 + col] == 1 {
                    row = bn;
                    println!("--col: {}, {}", bingo, row);
                    
                    if row != 999 {
                        let mut sum: i32 = 0;
                        for j in 25 * row..25 * (row + 1) {
                            if num_index[j] == 0 {
                                sum = sum + numbers[j];
                            }
                        }
                        marked_boards.push(row);
                        println!("{}: {} * {} = {}", marked_boards.len(), sum, bingo, sum * bingo);
                    }
                }
            }
        }
    }
}