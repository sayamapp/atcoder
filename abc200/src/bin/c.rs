use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        a: [u128; n],
    }
    let mut xs: Vec<u128> = vec![0; 200];
    for n in a {
        let idx = n % 200;
        xs[idx as usize] += 1;
    }

    let mut res = 0;
    for x in xs {
        if x > 0 {
            res += (x * (x - 1)) / 2;
        }
    }

    println!("{}", res);

}
