// use std::fs;

// fn main() {
//     let content = fs::read_to_string("input1.txt").expect("something went wrong");
//     let lines: Vec<&str> = content.split("\n").collect();
//     let mut sum: i32 = 0;

//     for line in lines {
//         let mut stack: Vec<char> = Vec::new();

//         for ch in line.chars() {
//             if ch == '{' || ch == '(' || ch == '[' || ch =='<' {
//                 stack.push(ch);
//             }
//             else {
//                 let poped = stack.pop().unwrap();
//                 if (ch == '}' && poped != '{') || (ch == ']' && poped != '[') || (ch == ')' && poped != '(') || (ch == '>' && poped != '<') {
//                     if ch == ')' {
//                         sum += 3;
//                     }
//                     if ch == ']' {
//                         sum += 57;
//                     }
//                     if ch == '}' {
//                         sum += 1197;
//                     }
//                     if ch == '>' {
//                         sum += 25137;
//                     }
//                 }
//             }

//         }
//     }

//     println!("{}", sum);
// }
use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut scores: Vec<i64> = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let len = line.chars().count();
        for i in 0..len {
            let ch: char = line.chars().nth(i).unwrap();
            if ch == '{' || ch == '(' || ch == '[' || ch =='<' {
                stack.push(ch);
            }
            else {
                let poped = stack.pop().unwrap();
                if (ch == '}' && poped != '{') || (ch == ']' && poped != '[') || (ch == ')' && poped != '(') || (ch == '>' && poped != '<') {
                    break;
                }
            }
            if i == len - 1 {
                let mut score: i64 = 0;
                stack.reverse();
                for each in stack.clone() {
                    if each == '(' {
                        score = score * 5 + 1;
                    }
                    if each == '[' {
                        score = score * 5 + 2;
                    }
                    if each == '{' {
                        score = score * 5 + 3;
                    }
                    if each == '<' {
                        score = score * 5 + 4;
                    }
                }
                scores.push(score);
            }
        }
    }

    scores.sort();
    println!("{:?}", scores);
    println!("{}", scores[scores.len()/2]);
}