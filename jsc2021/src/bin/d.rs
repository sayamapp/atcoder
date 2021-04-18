use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n : i128,
        p: i128,
    }
    let m: i128 = 10e9 as i128 + 7;
    let x = mod_pow(p - 1, n, m);
    let y = ((p - 1) * n) % m;

    println!("{}", x - y);
    println!("{} {} {}", x, y,(x - y + m) % m);
    
}

fn mod_pow(mut a: i128, mut n: i128, m: i128) -> i128 {
    let mut res = 1;
    while n > 0 {
        if n & 1 != 0 { res = res * a % m;}
        a = a * a % m;
        n >>= 1;
    }
    return res;
}