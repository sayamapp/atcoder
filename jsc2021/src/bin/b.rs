use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut res = Vec::new();
    for i in 1..=1000 {
        if !a.iter().any(|&x| x == i) && b.iter().any(|&x| x == i) {res.push(i);}
        if a.iter().any(|&x| x == i) && !b.iter().any(|&x| x == i) {res.push(i);}
    }
    for n in res {
        print!("{} ", n);
    }
        println!();
}
