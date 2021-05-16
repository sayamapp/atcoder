use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut memo = vec![Vec::new(); h + w - 1];

    for i in 0..h {
        for j in 0..w {
            memo[i + j].push(a[i][j]);
        }
    }

    let mut st = 0;
    let mut sa = 0;

    for (n, cs) in memo.iter().enumerate().skip(1) {
        if cs.iter().all(|c| *c == '+') {
            if n % 2 == 0 {
                sa += 1;
            } else {
                st += 1;
            }
        } else if cs.iter().all(|c| *c == '-') {
            if n % 2 == 0 {
                sa -= 1;
            } else {
                st -= 1;
            }
        }
    }

    if st > sa {
        println!("Takahashi");
    } else if st < sa {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
