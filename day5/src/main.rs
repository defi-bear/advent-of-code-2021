// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let mut array = vec![vec![0; 1000]; 1000];
//     let mut count = 0;

//     for line in lines {
//         let parts: Vec<&str> = line.split(" -> ").collect();
//         let part_1: Vec<&str> = parts[0].split(",").collect();
//         let x1: usize = part_1[0].parse().unwrap();
//         let y1: usize = part_1[1].parse().unwrap();
//         let part_2: Vec<&str> = parts[1].split(",").collect();
//         let x2: usize = part_2[0].parse().unwrap();
//         let y2: usize = part_2[1].parse().unwrap();

//         if x1 == x2 {
//             let min = y1.min(y2);
//             let max = y1.max(y2);
//             for i in min..max + 1 {
//                 array[x1][i] = array[x1][i] + 1; 
//             }
//         }
//         if y1 == y2 {
//             let min = x1.min(x2);
//             let max = x1.max(x2);
//             for i in min..max  + 1 {
//                 array[i][y1] = array[i][y1] + 1; 
//             }
//         }
//     }
//     for i in 0..1000 {
//         for j in 0..1000 {
//             if array[i][j] > 1 {
//                 count = count + 1;
//             }
//         }
//     }
//     println!("{}", count);
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut array = vec![vec![0; 1000]; 1000];
    let mut count = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let part_1: Vec<&str> = parts[0].split(",").collect();
        let x1: usize = part_1[0].parse().unwrap();
        let y1: usize = part_1[1].parse().unwrap();
        let part_2: Vec<&str> = parts[1].split(",").collect();
        let x2: usize = part_2[0].parse().unwrap();
        let y2: usize = part_2[1].parse().unwrap();
        println!("{}", line);
        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);
            for i in min..max + 1 {
                array[x1][i] = array[x1][i] + 1; 
            }
        } else if y1 == y2 {
            let min = x1.min(x2);
            let max = x1.max(x2);
            for i in min..max  + 1 {
                array[i][y1] = array[i][y1] + 1; 
            }
        } else if (x1 as i32 - x2 as i32).abs() == (y1 as i32 - y2 as i32).abs() {
            if x1 < x2 {
                for i in x1..x2 + 1 {
                    let mut j: usize = 0;
                    if y2 > y1 {
                        j = y1 + (i - x1);
                    }
                    if y1 > y2 {
                        j = y1 - (i - x1);
                    }
                    array[i][j] = array[i][j] + 1;
                }
            }
            if x1 > x2 {
                for i in x2..x1 + 1 {
                    let mut j: usize = 0;
                    if y2 > y1 {
                        j = y2 - (i - x2);
                    }
                    if y1 > y2 {
                        j = y2 + (i - x2);
                    }
                    array[i][j] = array[i][j] + 1;
                }
            }
        }
        
    }
    for i in 0..1000 {
        for j in 0..1000 {
            if array[i][j] > 1 {
                count = count + 1;
            }
        }
    }
    println!("{}", count);
}