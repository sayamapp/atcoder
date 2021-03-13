use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: u128,
    }
    let mut res = 0;

    if n >= 1_000 && n <= 999_999 {
        res = n - 1_000 + 1;
    }
    if n >= 1_000_000 && n <= 999_999_999 {
        res = (n - 1_000_000 + 1) * 2 + 999_000;
    }
    if n >= 1_000_000_000 && n <= 999_999_999_999 {
        res = (n - 1_000_000_000 + 1) * 3 + 999_000 + 999_000_000 * 2;
    }
    if n >= 1_000_000_000_000 && n <= 999_999_999_999_999 {
        res = (n - 1_000_000_000_000 + 1) * 4 + 999_000 + 999_000_000 * 2 + 999_000_000_000 * 3;
    }
    if n >= 1_000_000_000_000_000 {
        res = (n - 1_000_000_000_000_000 + 1) * 5 + 999_000 + 999_000_000 * 2 + 999_000_000_000 * 3 + 999_000_000_000_000 * 4;
    }
    println!("{}", res);
}
