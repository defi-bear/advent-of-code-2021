// use std::fs;
// const SIZE: usize = 40;

// fn recurrent(arr: &Vec<Vec<i32>>, cnt: &mut i32, pos: &mut usize, visited: &mut Vec<i32>) {
//     if *pos == 1 {
//         *cnt += 1;
//         return;
//     }
//     for i in 1..SIZE {
//         if arr[*pos][i] == 1 {
//             if (i <  21 && visited[i] == 0) || i > 20 {
//                 visited[*pos] += 1;
//                 recurrent(arr, cnt, &mut i.clone(), visited);
//                 visited[*pos] -= 1;
//             }
//         }
//     }
// }


// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let mut arr = vec![vec![0; SIZE]; SIZE];
//     let mut lower: usize = 2;
//     let mut upper: usize = 21;
//     let mut lower_arr: Vec<&str> = Vec::new();
//     let mut upper_arr: Vec<&str> = Vec::new();
//     lower_arr.push("start");
//     lower_arr.push("end");

//     for line in lines {
//         let splitted: Vec<&str> = line.split("-").collect();
//         let left = splitted[0];
//         let right = splitted[1];
//         let mut left_index: usize = 0;
//         let mut right_index: usize = 0;
//         if left == "start" {
//             left_index = 0;
//         } else if left == "end" {
//             left_index = 1;
//         } else {
//             let mut flag: bool = false;
//             for i in 2..lower_arr.len() {
//                 if left == lower_arr[i] {
//                     left_index = i;
//                     flag = true;
//                 }
//             }
//             if !flag {
//                 for i in 0..upper_arr.len() {
//                     if left == upper_arr[i] {
//                         left_index = i + 21;
//                         flag = true;
//                     }
//                 }
//                 if !flag {
//                     let mut cnt: usize = 0;
//                     for ch in left.chars() {
//                         if ch >= 'a' && ch <= 'z' {
//                             cnt += 1;
//                         }
//                     }
//                     // println!("left cnt: {}, {}", cnt, left.chars().count());
//                     if cnt == left.chars().count() {
//                         left_index = lower;
//                         lower += 1;
//                         lower_arr.push(left);
//                     } else {
//                         left_index = upper;
//                         upper += 1;
//                         upper_arr.push(left);
//                     }
//                 }
//             }
//         }
//         if right == "start" {
//             right_index = 0;
//         } else if right == "end" {
//             right_index = 1;
//         } else {
//             let mut flag: bool = false;
//             for i in 2..lower_arr.len() {
//                 if right == lower_arr[i] {
//                     right_index = i;
//                     flag = true;
//                 }
//             }
//             if !flag {
//                 for i in 0..upper_arr.len() {
//                     if right == upper_arr[i] {
//                         right_index = i + 21;
//                         flag = true;
//                     }
//                 }
//                 if !flag {
//                     let mut cnt: usize = 0;
//                     for ch in right.chars() {
//                         if ch >= 'a' && ch <= 'z' {
//                             cnt += 1;
//                         }
//                     }
//                     if cnt == right.chars().count() {
//                         right_index = lower;
//                         lower += 1;
//                         lower_arr.push(right);
//                     } else {
//                         right_index = upper;
//                         upper += 1;
//                         upper_arr.push(right);
//                     }
//                 }
//             }
//         }
//         println!("{}, {}, {}, {}",left, right, left_index, right_index);
//         arr[left_index][right_index] = 1;
//         arr[right_index][left_index] = 1;
//     }
//     let mut cnt: i32 = 0;
//     let mut pos: usize = 0;
//     let mut visited = vec![0; SIZE];
//     recurrent(&arr, &mut cnt, &mut pos, &mut visited);
//     println!("{}", cnt);
// }

use std::fs;
const SIZE: usize = 40;

fn recurrent(arr: &Vec<Vec<i32>>, cnt: &mut i32, pos: &mut usize, visited: &mut Vec<i32>, flag: &mut bool) {
    if *pos == 1 {
        *cnt += 1;
        return;
    }
    for i in 1..SIZE {
        if arr[*pos][i] == 1 {
            if (i <  21 && visited[i] == 0) || i > 20 {
                visited[*pos] += 1;
                recurrent(arr, cnt, &mut i.clone(), visited, flag);
                visited[*pos] -= 1;
            } else if !*flag {
                visited[*pos] += 1;
                *flag = true;
                recurrent(arr, cnt, &mut i.clone(), visited, flag);
                visited[*pos] -= 1;
                *flag = false;
            }
        }
    }
}


fn main() {
    let content = fs::read_to_string("input.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut arr = vec![vec![0; SIZE]; SIZE];
    let mut lower: usize = 2;
    let mut upper: usize = 21;
    let mut lower_arr: Vec<&str> = Vec::new();
    let mut upper_arr: Vec<&str> = Vec::new();
    lower_arr.push("start");
    lower_arr.push("end");

    for line in lines {
        let splitted: Vec<&str> = line.split("-").collect();
        let left = splitted[0];
        let right = splitted[1];
        let mut left_index: usize = 0;
        let mut right_index: usize = 0;
        if left == "start" {
            left_index = 0;
        } else if left == "end" {
            left_index = 1;
        } else {
            let mut flag: bool = false;
            for i in 2..lower_arr.len() {
                if left == lower_arr[i] {
                    left_index = i;
                    flag = true;
                }
            }
            if !flag {
                for i in 0..upper_arr.len() {
                    if left == upper_arr[i] {
                        left_index = i + 21;
                        flag = true;
                    }
                }
                if !flag {
                    let mut cnt: usize = 0;
                    for ch in left.chars() {
                        if ch >= 'a' && ch <= 'z' {
                            cnt += 1;
                        }
                    }
                    // println!("left cnt: {}, {}", cnt, left.chars().count());
                    if cnt == left.chars().count() {
                        left_index = lower;
                        lower += 1;
                        lower_arr.push(left);
                    } else {
                        left_index = upper;
                        upper += 1;
                        upper_arr.push(left);
                    }
                }
            }
        }
        if right == "start" {
            right_index = 0;
        } else if right == "end" {
            right_index = 1;
        } else {
            let mut flag: bool = false;
            for i in 2..lower_arr.len() {
                if right == lower_arr[i] {
                    right_index = i;
                    flag = true;
                }
            }
            if !flag {
                for i in 0..upper_arr.len() {
                    if right == upper_arr[i] {
                        right_index = i + 21;
                        flag = true;
                    }
                }
                if !flag {
                    let mut cnt: usize = 0;
                    for ch in right.chars() {
                        if ch >= 'a' && ch <= 'z' {
                            cnt += 1;
                        }
                    }
                    if cnt == right.chars().count() {
                        right_index = lower;
                        lower += 1;
                        lower_arr.push(right);
                    } else {
                        right_index = upper;
                        upper += 1;
                        upper_arr.push(right);
                    }
                }
            }
        }
        println!("{}, {}, {}, {}",left, right, left_index, right_index);
        arr[left_index][right_index] = 1;
        arr[right_index][left_index] = 1;
    }
    let mut cnt: i32 = 0;
    let mut pos: usize = 0;
    let mut visited = vec![0; SIZE];
    let mut flag: bool = false;
    recurrent(&arr, &mut cnt, &mut pos, &mut visited, &mut flag);
    println!("{}", cnt);
}