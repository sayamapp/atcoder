use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
fn main() {
    input! {
        n: Chars,
    }
    let mut que = VecDeque::from(n);

    let mut res = false;
    while que.len() < 20  {
        let n = Vec::from(que.clone());
        let mut rn = n.clone();
        rn.reverse();
        if n == rn {
            res = true;
            break;
        } else {
            que.push_front('0');
        }
    }
    println!("{}", if res {"Yes"}else{"No"});
}
