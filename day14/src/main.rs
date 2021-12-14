// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();

//     let mut ch_vec: Vec<char> = Vec::new();
//     let mut left_vec: Vec<&str> = Vec::new();
//     let mut right_vec: Vec<char> = Vec::new();
//     for i in 0..lines[0].chars().count() {
//         ch_vec.push(lines[0].chars().nth(i).unwrap());
//     }
//     for i in 2..lines.len() {
//         let line = lines[i];
//         let splitted: Vec<&str> = line.split(" -> ").collect();
//         left_vec.push(splitted[0]);
//         let right_ch: char = splitted[1].chars().nth(0).unwrap();
//         right_vec.push(right_ch);
//     }
//     for _i in 0..10 {
//         let mut ch_temp: Vec<char> = Vec::new();
//         for j in 0..ch_vec.len() - 1 {
//             let mut flag: bool = false;
//             for k in 0..left_vec.len() {
//                 if ch_vec[j] == left_vec[k].chars().nth(0).unwrap() && ch_vec[j + 1] == left_vec[k].chars().nth(1).unwrap() {
//                     ch_temp.push(ch_vec[j]);
//                     ch_temp.push(right_vec[k]);
//                     flag = true;
//                 }
//             }
//             if !flag {
//                 ch_temp.push(ch_vec[j]);
//             }
//         }
//         ch_temp.push(ch_vec[ch_vec.len() - 1]);
//         ch_vec = ch_temp;
//     }
//     println!("{}", ch_vec.len());

//     let mut ch_list: Vec<char> = Vec::new();
//     let mut count: Vec<i64> = Vec::new();
//     for i in 0..ch_vec.len() {
//         let mut flag: bool = false;
//         for j in 0..ch_list.len() {
//             if ch_list[j] == ch_vec[i] {
//                 count[j] += 1;
//                 flag = true;
//             }
//         }
//         if !flag {
//             ch_list.push(ch_vec[i]);
//             count.push(1);
//         }
//     }
//     count.sort();
//     println!("{:?}, {}", count, count[count.len() - 1] - count[0]);
// }
use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut left_vec: Vec<&str> = Vec::new();
    let mut right_vec: Vec<char> = Vec::new();
    let mut count: Vec<i64> = Vec::new();
    let mut ch_vec: Vec<char> = Vec::new();
    for i in 0..lines[0].chars().count() {
        ch_vec.push(lines[0].chars().nth(i).unwrap());
    }
    for i in 2..lines.len() {
        let line = lines[i];
        let splitted: Vec<&str> = line.split(" -> ").collect();
        left_vec.push(splitted[0]);
        let right_ch: char = splitted[1].chars().nth(0).unwrap();
        right_vec.push(right_ch);
        count.push(0);
    }

    for j in 0..ch_vec.len() - 1 {
        for k in 0..left_vec.len() {
            if ch_vec[j] == left_vec[k].chars().nth(0).unwrap() && ch_vec[j + 1] == left_vec[k].chars().nth(1).unwrap() {
                count[k] += 1;
            }
        }
    }

    for _i in 0..40 {
        let mut new_count = vec![0; left_vec.len()];

        for j in 0..left_vec.len() {
            if count[j] > 0 {
                for k in 0..left_vec.len() {
                    if left_vec[j].chars().nth(0).unwrap() == left_vec[k].chars().nth(0).unwrap() && right_vec[j] == left_vec[k].chars().nth(1).unwrap() {
                        new_count[k] += count[j];
                    }
                    if left_vec[j].chars().nth(1).unwrap() == left_vec[k].chars().nth(1).unwrap() && right_vec[j] == left_vec[k].chars().nth(0).unwrap() {
                        new_count[k] += count[j];
                    }
                }
            }
        }
        count = new_count;
    }
    
    let mut ch_list: Vec<char> = Vec::new();
    let mut counts: Vec<i64> = Vec::new();
    for i in 0..left_vec.len() {
        let mut flag: bool = false;
        for j in 0..ch_list.len() {
            if ch_list[j] == left_vec[i].chars().nth(0).unwrap() {
                counts[j] += count[i];
                flag = true;
            }
        }
        if !flag {
            ch_list.push(left_vec[i].chars().nth(0).unwrap());
            counts.push(count[i]);
        }
        flag = false;
        for j in 0..ch_list.len() {
            if ch_list[j] == left_vec[i].chars().nth(1).unwrap() {
                counts[j] += count[i];
                flag = true;
            }
        }
        if !flag {
            ch_list.push(left_vec[i].chars().nth(1).unwrap());
            counts.push(count[i]);
        }

    }
    for i in 0..ch_list.len() {
        if ch_list[i] == lines[0].chars().nth(0).unwrap() || ch_list[i] == lines[0].chars().nth(lines[0].chars().count() - 1).unwrap() {
            counts[i] = (counts[i] + 1) / 2;
            println!("{}", ch_list[i]);
        } else {
            counts[i] = counts[i] / 2;
        }
    }
    counts.sort();
    println!("{:?}, {}", counts, counts[counts.len() - 1] - counts[0]);
}