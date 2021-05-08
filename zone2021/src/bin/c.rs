use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        a: [[usize; 5]; n],
    }
    
    let mut maxs = vec![0, 0, 0, 0, 0];
    for xs in a {
        for i in 0..5 {
            maxs[i] = maxs[i].max(xs[i]);
        }
    }
    println!("{:?}", maxs);
    let res = maxs.iter().min().unwrap();
    println!("{}", res);
}
