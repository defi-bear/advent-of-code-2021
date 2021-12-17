fn main() {
    const MIN_X: i32 = 169;// 20;
    const MAX_X: i32 = 206;// 30;
    const MIN_Y: i32 = -108;// -10;
    const MAX_Y: i32 = -68;// -5;

    let mut sum: i32 = 0;
    for i in 0..MAX_X + 1 {
        for j in MIN_Y..MIN_Y.abs() + 1 {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut xx: i32 = i.clone();
            let mut yy: i32 = j.clone();
            loop {
                x += xx;
                y += yy;
                if x >= MIN_X && x <= MAX_X && y >=MIN_Y && y <= MAX_Y {
                    sum += 1;
                    println!("{}, {}", i, j);
                    break;
                } else if x > MAX_X || y < MIN_Y {
                    break;
                }
                if xx > 0 {
                    xx -= 1;
                }
                yy -= 1;
            }
        }
    }
    println!("{}", sum);
}
