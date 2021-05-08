use itertools::Itertools;
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
fn main() {
    input! {
        s: Chars,
    }
    let mut t = VecDeque::new();
    let mut is_reverse = false;
    for c in s {
        if c == 'R' {
            is_reverse = !is_reverse;
        } else {
            if !is_reverse {
                if let Some(pt) = t.pop_back() {
                    if c != pt {
                        t.push_back(pt);
                        t.push_back(c);
                    }
                } else {
                    t.push_back(c);
                }
            } else {
                if let Some(nt) = t.pop_front() {
                    if c != nt {
                        t.push_front(nt);
                        t.push_front(c);
                    }
                } else {
                    t.push_front(c);
                }
            }
        }
    }

    let mut res = Vec::from(t);
    if is_reverse {
        res.reverse();
    }
    let res: String = res.iter().collect();
    println!("{}", res);
}
