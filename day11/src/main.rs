// use std::fs;

// const SIZE: usize = 10;

// fn recurrent(arr: &mut Vec<Vec<i32>>, cnt: &mut i32, flag: &mut bool) {
//     if *flag {
//         for i in 0..SIZE {
//             for j in 0..SIZE {
//                 arr[i][j] += 1;
//             }
//         }
//         *flag = false;
//     }
//     let mut ninecount = 0;
//     for i in 0..SIZE {
//         for j in 0..SIZE {
//             if arr[i][j] > 9 && arr[i][j] < 100 {
//                 arr[i][j] += 100;
//                 ninecount += 1;
//                 *cnt += 1;
//                 if i > 0 && j > 0 { arr[i - 1][j - 1] += 1; }
//                 if i > 0 { arr[i - 1][j] += 1; }
//                 if j > 0 { arr[i][j - 1] += 1; }
//                 if j > 0 && i < SIZE - 1 { arr[i + 1][j - 1] += 1; }
//                 if i > 0 && j < SIZE - 1 { arr[i - 1][j + 1] += 1; }
//                 if i < SIZE - 1 { arr[i + 1][j] += 1; }
//                 if j < SIZE - 1 { arr[i][j + 1] += 1; }
//                 if j < SIZE - 1 && i < SIZE -1 { arr[i + 1][j + 1] += 1; }
//             }
//         }
//     }
//     // println!("{:?}", arr);
//     if ninecount == 0 {
//         for i in 0..SIZE {
//             for j in 0..SIZE {
//                 if arr[i][j] > 9 {
//                     arr[i][j] = 0;
//                 }
//             }
//         }
//         return;
//     }
//     recurrent(arr, cnt, flag);
// }

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();

//     let mut arr = vec![vec![0; SIZE]; SIZE];
//     for i in 0..SIZE {
//         let line = lines[i];
//         for j in 0..SIZE {
//             let ch = line.chars().nth(j).unwrap();
//             arr[i][j] = ch as i32 - 48;
//         }
//     }

//     let mut cnt: i32 = 0;
//     for _i in 0..100 {
//         let mut flag: bool = true;
//         recurrent(&mut arr, &mut cnt, &mut flag);
//     }

//     println!("{:?} {}", arr, cnt);
// }

use std::fs;

const SIZE: usize = 10;

fn recurrent(arr: &mut Vec<Vec<i32>>, cnt: &mut i32, flag: &mut bool, one_cnt: &mut i32) {
    if *flag {
        for i in 0..SIZE {
            for j in 0..SIZE {
                arr[i][j] += 1;
            }
        }
        *flag = false;
    }
    let mut ninecount = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if arr[i][j] > 9 && arr[i][j] < 100 {
                arr[i][j] += 100;
                ninecount += 1;
                *cnt += 1;
                if i > 0 && j > 0 { arr[i - 1][j - 1] += 1; }
                if i > 0 { arr[i - 1][j] += 1; }
                if j > 0 { arr[i][j - 1] += 1; }
                if j > 0 && i < SIZE - 1 { arr[i + 1][j - 1] += 1; }
                if i > 0 && j < SIZE - 1 { arr[i - 1][j + 1] += 1; }
                if i < SIZE - 1 { arr[i + 1][j] += 1; }
                if j < SIZE - 1 { arr[i][j + 1] += 1; }
                if j < SIZE - 1 && i < SIZE -1 { arr[i + 1][j + 1] += 1; }
            }
        }
    }
    // println!("{:?}", arr);
    if ninecount == 0 {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if arr[i][j] > 9 {
                    *one_cnt += 1;
                    arr[i][j] = 0;
                }
            }
        }
        return;
    }
    recurrent(arr, cnt, flag, one_cnt);
}

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut arr = vec![vec![0; SIZE]; SIZE];
    for i in 0..SIZE {
        let line = lines[i];
        for j in 0..SIZE {
            let ch = line.chars().nth(j).unwrap();
            arr[i][j] = ch as i32 - 48;
        }
    }

    let mut cnt: i32 = 0;
    for i in 0..1000 {
        let mut flag: bool = true;
        let mut one_cnt: i32 = 0;
        recurrent(&mut arr, &mut cnt, &mut flag, &mut one_cnt);
        if one_cnt == 100 {
            println!("{}", i + 1);
            println!("{:?}", arr);
            return;
        }
    }
}