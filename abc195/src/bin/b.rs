use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    }
    let mut min = None;
    let mut max = None;
    for i in 1..=(w * 1000) {
        let w = w * 1000;
        let ave = w / i;
        // if ave <= b && i * b >= w && min == None {min = Some(i)};
        // if ave <= b && ave >= a && i * a >= w && i * b  <= w {max = Some(i)};
        if ave <= b && i * b >= w && min == None{min = Some(i)};
        if ave <= b && i * b >= w && ave >= a && i * a <= w {max = Some(i)};
        if ave < a {break;}
    }
    let res =
        if min == None || max == None {"UNSATISFIABLE".to_string()}
        else {format!("{} {}", min.unwrap(), max.unwrap())};

    println!("{}", res);
}
