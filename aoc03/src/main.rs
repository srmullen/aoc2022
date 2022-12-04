use itertools::Itertools;
use std::collections::HashSet;

fn part1() {
    let answer = include_str!("input.txt")
        .lines()
        .map(|s| {
            // Split the string in half.
            let count = s.chars().count();
            s.split_at(count / 2)
        })
        .map(|(s1, s2)| {
            // find which character is the same
            for c1 in s1.chars() {
                for c2 in s2.chars() {
                    if c1 == c2 {
                        return c1;
                    }
                }
            }
            panic!("No matching character");
        }).map(|c| {
            let ascii = c as u32;
            if ascii > 96 {
                ascii - 96
            } else {
                ascii - 38
            }
        }).reduce(|a, b| a + b);
    
    println!("{:?}", answer);
}

fn part2() {
    let answer = include_str!("input.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            // println!("{:?}", chunk);
            let group = chunk.into_iter().map(|s| {
                s.chars().unique().collect::<String>()
            }).join("");
            // println!("{}", group);
            group
        })
        .map(|s| {
            let res = s.chars().counts().into_iter().find(|(_, val)| {
                val.eq(&3)
            });
            match res {
                Some((key, _)) => key,
                None => panic!(),
            }
        }).map(|c| {
            let ascii = c as u32;
            if ascii > 96 {
                ascii - 96
            } else {
                ascii - 38
            }
        })
        .reduce(|a, b| a + b);

    println!("Answer: {:?}", answer);
}

fn main() {
    // part1();
    part2();    
}
