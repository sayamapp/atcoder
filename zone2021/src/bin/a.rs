use std::io::Chain;

use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        s: Chars,
    }
    let res = s.windows(4).filter(|&c| {
        c[0] == 'Z' && c[1] == 'O' && c[2] == 'N' && c[3] == 'e'
    }).count();

    println!("{}", res);
}

