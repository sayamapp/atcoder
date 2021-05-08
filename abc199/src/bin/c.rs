use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        query: [(usize, usize, usize); q],
    }
    let mut flip = false;

    for (f, a, b) in query {
        if f == 1 {
            if !flip {
                s = f2(s, a - 1, b - 1);
            } else {
                let c = if a - 1 >= n {a - n} else {a + n};
                let d = if b - 1 >= n {b - n} else {b + n};
                s = f2(s, c - 1, d - 1);
            }
        } else {
            flip = !flip;
        }
    }
    if flip {
        let mut t = s.split_off(n);
        t.append(&mut s);
        s = t;
    }
    let res: String = s.iter().collect();
    println!("{}", res);
}

fn f2(mut s: Vec<char>, a: usize, b: usize) -> Vec<char> {
    let x = s[a];
    s[a] = s[b];
    s[b] = x;
    s
}
