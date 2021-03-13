use proconio::input;
use proconio::marker::*;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(usize, usize); n],
        x: [usize; m],
        lr: [(Usize1, Usize1); q],
    }

    for (l, r) in lr {
        let mut boxes = Vec::new();
        for i in 0..m {
            if i < l || r < i {
                boxes.push(x[i]);
            }
        }

        let mut res = 0;
        let x = (0..n).permutations(boxes.len()).collect_vec();
        for j in 0..x.len() {
            let mut max=0;
            for i in 0..boxes.len() {
                if (wv[x[j][i]]).0 <= boxes[i] {
                    max += wv[i].1;
                }
            }
            res = res.max(max);
        }
        println!("{}", res);
    }

}
