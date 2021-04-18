use proconio::input;
use proconio::marker::*;
use num::integer;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut max= 0;
    for i in 1..b {
        let x = if a % i == 0 {a / i} else {a / i + 1};
        if i * (x + 1) <= b {max = i;}
    }
    println!("{}", max);
}
