use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n],
    }
    st.sort_by(|a, b| (b.1).cmp(&a.1));
    println!("{}", st[1].0);
}
