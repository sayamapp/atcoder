use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let max_a = a.iter().max().unwrap();
    let min_b = b.iter().min().unwrap();
    let res = if min_b < max_a {0} else {min_b - max_a + 1};
    println!("{}", res);
}
