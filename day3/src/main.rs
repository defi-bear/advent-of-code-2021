// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let len = lines[0].chars().count();
//     println!("{}", len);
//     let mut vec: Vec<i32> = vec![0; len];

//     for line in lines {
//         for (j, ch) in line.chars().enumerate() {
//             if ch == '1' {
//                 vec[j] = vec[j] + 1;
//             } else {
//                 vec[j] = vec[j] - 1;
//             }
//         }
//     }

//     let mut positive = String::from("");
//     let mut negative = String::from("");
//     for each in vec {
//         if each > 0 {
//             positive.push('1');
//             negative.push('0');
//         } else {
//             positive.push('0');
//             negative.push('1');
//         }
//     }
//     let pos_num = isize::from_str_radix(positive.as_str(), 2).unwrap();
//     let neg_num = isize::from_str_radix(negative.as_str(), 2).unwrap();

//     println!("{}", pos_num * neg_num);
    
// }

use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let len = lines[0].chars().count();

    let mut oxygen: Vec<&str> = lines.clone();
    let mut co2: Vec<&str> = lines;

    for i in 0..len {
        let mut cnt = 0;
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for j in 0..oxygen.len() {
            if oxygen[j].as_bytes()[i] == b'1' {
                cnt = cnt + 1;
                pos.push(oxygen[j]);
            } else {
                cnt = cnt - 1;
                neg.push(oxygen[j]);
            }
        }
        if cnt >= 0 {
            oxygen = pos;
        } else {
            oxygen = neg;
        }
        
        if co2.len() > 1 {
            let mut cpos = Vec::new();
            let mut cneg = Vec::new();
            cnt = 0;
            for j in 0..co2.len() {
                if co2[j].as_bytes()[i] == b'1' {
                    cnt = cnt + 1;
                    cpos.push(co2[j]);
                } else {
                    cnt = cnt - 1;
                    cneg.push(co2[j]);
                }
            }
            if cnt >= 0 {
                co2 = cneg;
            } else {
                co2 = cpos;
            }
        }
    }
    println!("{}, {}", oxygen[0], co2[0]);

    let pos_num = isize::from_str_radix(oxygen[0], 2).unwrap();
    let neg_num = isize::from_str_radix(co2[0], 2).unwrap();

    println!("{}", pos_num * neg_num);
}