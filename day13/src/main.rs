// use std::fs;

// const SIZE: usize = 1500;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();

//     let mut arr = vec![vec![0; SIZE]; SIZE];
//     let mut axios_arr: Vec<&str> = Vec::new();
//     let mut pos_arr: Vec<usize> = Vec::new();
//     for line in lines {
//         if line.chars().count() > 13 {
//             let split: Vec<&str> = line.split(" ").collect();
//             let two: Vec<&str> = split[2].split("=").collect();
//             let left = two[0];
//             let right: usize = two[1].parse().unwrap();
//             axios_arr.push(left);
//             pos_arr.push(right);
//         } else if line.chars().count() > 0 {
//             let two: Vec<&str> = line.split(",").collect();
//             let left: usize = two[0].parse().unwrap();
//             let right: usize = two[1].parse().unwrap();
//             arr[right][left] = 1;
//             // println!("{}, {}", left, right);
//         }
//     }
//     let mut x_max = SIZE;
//     let mut y_max = SIZE;
//     // println!("{:?}", arr[14]);
//     for i in 0..1 {
//         let y = pos_arr[i];
//         if axios_arr[i] == "y" {
//             y_max = pos_arr[i];
//             for i in 0..x_max {
//                 for j in 0..y_max {
//                     if (y - j) + y < SIZE && arr[(y - j) + y][i] == 1 {
//                         arr[j][i] = 1;
//                     }
//                 }
//             }
//         } else if axios_arr[i] == "x" {
//             x_max = pos_arr[i];
//             for i in 0..x_max {
//                 for j in 0..y_max {
//                     if (y - i) + y < SIZE && arr[j][(y - i) + y] == 1{
//                         arr[j][i] = 1;
//                     }
//                 }
//             }
//         }
//     }
//     // println!("{:?}", arr);
//     println!("{}, {}", x_max, y_max);
//     let mut cnt: i32 = 0;
//     for i in 0..x_max {
//         for j in 0..y_max {
//             if arr[j][i] == 1 {
//                 cnt += 1;
//             }
//         }
//     }
//     println!("{}", cnt);
// }
use std::fs;

const SIZE: usize = 1500;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut arr = vec![vec![0; SIZE]; SIZE];
    let mut axios_arr: Vec<&str> = Vec::new();
    let mut pos_arr: Vec<usize> = Vec::new();
    for line in lines {
        if line.chars().count() > 13 {
            let split: Vec<&str> = line.split(" ").collect();
            let two: Vec<&str> = split[2].split("=").collect();
            let left = two[0];
            let right: usize = two[1].parse().unwrap();
            axios_arr.push(left);
            pos_arr.push(right);
        } else if line.chars().count() > 0 {
            let two: Vec<&str> = line.split(",").collect();
            let left: usize = two[0].parse().unwrap();
            let right: usize = two[1].parse().unwrap();
            arr[right][left] = 1;
            // println!("{}, {}", left, right);
        }
    }
    let mut x_max = SIZE;
    let mut y_max = SIZE;
    // println!("{:?}", arr[14]);
    for i in 0..pos_arr.len() {
        let y = pos_arr[i];
        if axios_arr[i] == "y" {
            y_max = pos_arr[i];
            for i in 0..x_max {
                for j in 0..y_max {
                    if (y - j) + y < SIZE && arr[(y - j) + y][i] == 1 {
                        arr[j][i] = 1;
                    }
                }
            }
        } else if axios_arr[i] == "x" {
            x_max = pos_arr[i];
            for i in 0..x_max {
                for j in 0..y_max {
                    if (y - i) + y < SIZE && arr[j][(y - i) + y] == 1{
                        arr[j][i] = 1;
                    }
                }
            }
        }
    }
    // println!("{:?}", arr);
    println!("{}, {}", x_max, y_max);
    let mut cnt: i32 = 0;
    for j in 0..y_max {
        let mut my_arr: Vec<char> = Vec::new();
        for i in 0..x_max {
            if arr[j][i] == 1 {
                my_arr.push('#');
            } else {
                my_arr.push('.');
            }
        }
        println!("{:?}", my_arr);
    }
}