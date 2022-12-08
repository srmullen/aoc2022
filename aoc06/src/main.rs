use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};


fn part1() {
    let chars: Vec<char> = include_str!("input.txt")
        .chars()
        .collect();
    let answer = chars.windows(4)
        .fold_while(0, |acc, sig| {
            if sig.into_iter().unique().join("") == sig.into_iter().join("") {
                Done(acc + 4)
            } else {
                Continue(acc + 1)
            }
        });
    println!("Answer: {:?}", answer);
}

fn part2() {
    let chars: Vec<char> = include_str!("input.txt")
        .chars()
        .collect();
    let answer = chars.windows(14)
        .fold_while(0, |acc, sig| {
            if sig.into_iter().unique().join("") == sig.into_iter().join("") {
                Done(acc + 14)
            } else {
                Continue(acc + 1)
            }
        });
    println!("Answer: {:?}", answer);
}

fn main() {
    // part1();
    part2();
}
