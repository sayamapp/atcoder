use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut xs = vec![0; 200];

    for n in &a {
        let idx = n % 200;
        xs[idx as usize] += 1;
    } 

    let mut flag = false;

    for i in 0..200 {
        if xs[i] > 1 {
            flag = true;
            let x = i;
            let mut res = Vec::new();
            for n in a.iter().enumerate() {
                if n.1 % 200 == x {
                    res.push(n.0);
                }
            }
            println!("Yes");
            println!("1 {}", res[0] + 1);
            println!("1 {}", res[1] + 1);
        }
    }

    if !flag {

        let mut memo = vec![false; 200];
        let mut memo2 = Vec::new();

        rec(&mut memo, &mut memo2, a, 0);
    }

}

fn rec(memo: Vec<bool>, memo2: Vec<usize>, a: Vec<usize>, n: usize, s: usize) {
    if n < a.len() {
        if s != 0 {
            if memo[s]
        }
    }

}
