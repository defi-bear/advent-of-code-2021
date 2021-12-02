// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let lines = content.split("\n");
//     let mut hor: i32 = 0;
//     let mut ver: i32 = 0;

//     for line in lines {
//         let split: Vec<&str> = line.split(" ").collect();
//         let num: i32 = split[1].parse().unwrap();

//         if split[0] == "forward" {
//             hor = hor + num;
//         } else if split[0] == "down" {
//             ver = ver + num;
//         } else {
//             ver = ver - num;
//         }
//     }
//     println!("{}", hor * ver);
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("something went wrong");
    let lines = content.split("\n");
    let mut hor: i32 = 0;
    let mut aim: i32 = 0;
    let mut ver: i32 = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let num: i32 = split[1].parse().unwrap();

        if split[0] == "forward" {
            hor = hor + num;
            ver = ver + aim * num;
        } else if split[0] == "down" {
            aim = aim + num;
        } else {
            aim = aim - num;
        }
    }
    println!("{}", hor * ver);
}