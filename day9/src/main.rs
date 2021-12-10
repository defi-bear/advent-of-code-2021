// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let y_size = lines.len();

//     let x_size = lines[0].chars().count();

//     let mut arr = vec![vec![0; x_size]; y_size];
//     for i in 0..y_size {
//         let line = lines[i];
//         for j in 0..x_size {
//             let ch = line.chars().nth(j).unwrap();
//             arr[i][j] = ch as i32 - 48;
//         }
//     }

//     let mut sum: i32 = 0;
//     for i in 0..y_size {
//         for j in 0..x_size {
//             let mut cnt = 0;
//             let current = arr[i][j];
//             if i != 0 {
//                 if arr[i - 1][j] > current {
//                     cnt += 1;
//                 }
//             } else {
//                 cnt += 1;
//             }
//             if i != y_size - 1 {
//                 if arr[i + 1][j] >current {
//                     cnt += 1;
//                 }
//             } else {
//                 cnt += 1;
//             }
//             if j != 0 {
//                 if arr[i][j - 1] > current {
//                     cnt += 1;
//                 }
//             } else {
//                 cnt += 1;
//             }
//             if j != x_size - 1 {
//                 if arr[i][j + 1] >current {
//                     cnt += 1;
//                 }
//             } else {
//                 cnt += 1;
//             }
//             if cnt == 4 {
//                 sum = sum + current + 1;
//             }
//         }
//     }
//     println!("{}", sum);
// }
use std::fs;

fn recurrent(i: usize, j: usize, x_size: usize, y_size: usize, arr: &Vec<Vec<i32>>, cnt: &mut i32, visited: &mut Vec<Vec<i32>>) {
    // println!("{}, {}, {}", i, j, arr[i][j]);
    if arr[i][j] == 9 {
        return
    }
    visited[i][j] = 1;
    if i != 0 && arr[i][j] < arr[i - 1][j] && visited[i - 1][j] != 1 && arr[i - 1][j] != 9 {
        *cnt += 1;
        recurrent(i - 1, j, x_size, y_size, &arr, cnt , visited);
    }
    if i != x_size - 1 && arr[i][j] < arr[i + 1][j] && visited[i + 1][j] != 1 && arr[i + 1][j] != 9 {
        *cnt += 1;
        recurrent(i + 1, j, x_size, y_size, &arr, cnt, visited);
    }
    if j != 0 && arr[i][j] < arr[i][j - 1] && visited[i][j - 1] != 1 && arr[i][j - 1] != 9 {
        *cnt += 1;
        recurrent(i, j - 1, x_size, y_size, &arr, cnt, visited);
    }
    if j != y_size - 1 && arr[i][j] < arr[i][j + 1] && visited[i][j + 1] != 1 && arr[i][j + 1] != 9 {
        *cnt += 1;
        recurrent(i, j + 1, x_size, y_size, &arr, cnt, visited);
    }
}

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let y_size = lines.len();

    let x_size = lines[0].chars().count();

    let mut arr = vec![vec![0; x_size]; y_size];
    for i in 0..y_size {
        let line = lines[i];
        for j in 0..x_size {
            let ch = line.chars().nth(j).unwrap();
            arr[i][j] = ch as i32 - 48;
        }
    }

    let mut basin: Vec<i32> = Vec::new();
    for i in 0..y_size {
        for j in 0..x_size {
            let mut cnt = 0;
            let current = arr[i][j];
            if i != 0 {
                if arr[i - 1][j] > current {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
            if i != y_size - 1 {
                if arr[i + 1][j] >current {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
            if j != 0 {
                if arr[i][j - 1] > current {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
            if j != x_size - 1 {
                if arr[i][j + 1] >current {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
            if cnt == 4 {
                let mut count: i32 = 1;
                let mut visited = vec![vec![0; x_size]; y_size];
                recurrent(i, j, y_size, x_size, &arr, &mut count, &mut visited);
                basin.push(count);
            }
        }
    }
    basin.sort();
    basin.reverse();
    println!("{}", basin[0] * basin[1] * basin[2]);
}