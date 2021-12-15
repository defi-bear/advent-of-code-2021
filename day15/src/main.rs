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
    
//     let mut visited = vec![vec![-1; x_size]; y_size];

//     visited[0][0] = arr[0][0];
//     loop {
//         let mut x: usize = 0;
//         let mut y: usize = 0;
//         let mut min: i32 = 10000;
//         for i in 0..y_size {
//             for j in 0..x_size {
//                 if min > visited[i][j] && visited[i][j] != -1 && visited[i][j] != -2 {
//                     x = i;
//                     y = j;
//                     min = visited[i][j];
//                 }
//             }
//         }
//         if x > 0 && visited[x - 1][y] != -2 && (visited[x - 1][y] == -1 || visited[x - 1][y] > visited[x][y] + arr[x - 1][y]) {
//             visited[x - 1][y] = visited[x][y] + arr[x - 1][y];
//         }
//         if y > 0 && visited[x][y - 1] != -2 && (visited[x][y - 1] == -1 || visited[x][y - 1] > visited[x][y] + arr[x][y - 1]) {
//             visited[x][y - 1] = visited[x][y] + arr[x][y - 1];
//         }
//         if x < y_size - 1 && visited[x + 1][y] != -2 && (visited[x + 1][y] == -1 || visited[x + 1][y] > visited[x][y] + arr[x + 1][y]) {
//             visited[x + 1][y] = visited[x][y] + arr[x + 1][y];
//         }
//         if y < x_size - 1 && visited[x][y + 1] != -2 && (visited[x][y + 1] == -1 || visited[x][y + 1] > visited[x][y] + arr[x][y + 1]) {
//             visited[x][y + 1] = visited[x][y] + arr[x][y + 1];
//         }
//         visited[x][y] = -2;
//         if visited[y_size - 1][x_size - 1] != -1 {
//             println!("{}", visited[y_size - 1][x_size - 1] - arr[0][0]);
//             break;
//         }
//     }
// }
use std::fs;

fn main() {
    let content = fs::read_to_string("input1.txt").expect("something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();

    let y_size = lines.len();
    let x_size = lines[0].chars().count();
    let mut arr = vec![vec![0; x_size * 5]; y_size * 5];
    for i in 0..y_size {
        let line = lines[i];
        for j in 0..x_size {
            let ch = line.chars().nth(j).unwrap();
            arr[i][j] = ch as i32 - 48;
        }
    }
    for i in 0..y_size * 5 {
        for j in 0..x_size * 5 {
            if arr[i][j] == 0 {
                arr[i][j] = arr[i % y_size][j % x_size] + ((i - i % y_size) / y_size + (j - j % x_size) / x_size ) as i32;
                if arr[i][j] > 9 {
                    arr[i][j] = arr[i][j] - 9;
                }
            }
        }
    }
    
    let mut visited = vec![vec![-1; x_size * 5]; y_size * 5];
    let mut queue: Vec<i32> = Vec::new();
    let mut xlist: Vec<usize> = Vec::new();
    let mut ylist: Vec<usize> = Vec::new();
    
    visited[0][0] = 0;
    queue.push(visited[0][0]);
    xlist.push(0);
    ylist.push(0);
    loop {
        let mut t: usize = 0;
        let mut min: i32 = 1000000;
        for i in 0..queue.len() {
            if min > visited[xlist[i]][ylist[i]] {
                t = i;
                min = visited[xlist[i]][ylist[i]];
            }
        }
        let x = xlist[t];
        let y = ylist[t];
        // println!("{}, {}, {}", x, y, visited[x][y]);
        if x > 0 && visited[x - 1][y] != -2 && (visited[x - 1][y] == -1 || visited[x - 1][y] > visited[x][y] + arr[x - 1][y]) {
            visited[x - 1][y] = visited[x][y] + arr[x - 1][y];
            queue.push(visited[x-1][y]);
            xlist.push(x - 1);
            ylist.push(y);
        }
        if y > 0 && visited[x][y - 1] != -2 && (visited[x][y - 1] == -1 || visited[x][y - 1] > visited[x][y] + arr[x][y - 1]) {
            visited[x][y - 1] = visited[x][y] + arr[x][y - 1];
            queue.push(visited[x][y - 1]);
            xlist.push(x);
            ylist.push(y - 1);
        }
        if x < y_size*5 - 1 && visited[x + 1][y] != -2 && (visited[x + 1][y] == -1 || visited[x + 1][y] > visited[x][y] + arr[x + 1][y]) {
            visited[x + 1][y] = visited[x][y] + arr[x + 1][y];
            queue.push(visited[x+1][y]);
            xlist.push(x + 1);
            ylist.push(y);
        }
        if y < x_size*5 - 1 && visited[x][y + 1] != -2 && (visited[x][y + 1] == -1 || visited[x][y + 1] > visited[x][y] + arr[x][y + 1]) {
            visited[x][y + 1] = visited[x][y] + arr[x][y + 1];
            queue.push(visited[x][y + 1]);
            xlist.push(x);
            ylist.push(y + 1);
        }
        visited[x][y] = -2;
        queue.remove(t);
        xlist.remove(t);
        ylist.remove(t);
        if visited[y_size*5 - 1][x_size*5 - 1] != -1 {
            println!("{}", visited[y_size*5 - 1][x_size*5 - 1]);
            break;
        }
    }
}