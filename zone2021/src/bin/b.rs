use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        ud: f64,
        uh: f64,
        dh: [(f64, f64); n],
    }
    let mut res = 0.0 as f32;
    for (d, h) in dh {
        let dx = ud - d;
        let dy = uh - h;
        let a = dy / dx;
        let h = uh - a * ud;
        res = res.max(h as f32);
    }
    println!("{}", res);
}
