use itertools::Itertools;

//         [M]     [B]             [N]
// [T]     [H]     [V] [Q]         [H]
// [Q]     [N]     [H] [W] [T]     [Q]
// [V]     [P] [F] [Q] [P] [C]     [R]
// [C]     [D] [T] [N] [N] [L] [S] [J]
// [D] [V] [W] [R] [M] [G] [R] [N] [D]
// [S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
// [N] [M] [F] [D] [R] [C] [W] [T] [M]
//  1   2   3   4   5   6   7   8   9 

fn part1() {
    let mut stacks = [
        vec!["N", "S", "D", "C", "V", "Q", "T"],
        vec!["M", "F", "V"],
        vec!["F", "Q", "W", "D", "P", "N", "H", "M"],
        vec!["D", "Q", "R", "T", "F"],
        vec!["R", "F", "M", "N", "Q", "H", "V", "B"],
        vec!["C", "F", "G", "N", "P", "W", "Q"],
        vec!["W", "F", "R", "L", "C", "T"],
        vec!["T", "Z", "N", "S"],
        vec!["M", "S", "D", "J", "R", "Q", "H", "N"],
    ];

    include_str!("input.txt")
        .lines()
        .map(|s| {
            let (_, count, _, from, _, to) = s.split_whitespace().next_tuple().unwrap();
            (count.parse::<usize>().unwrap(), from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
        })
        .for_each(|(count, from, to)| {
            for _ in 0..count {
                let tmp = stacks[from-1].pop().unwrap();
                stacks[to-1].push(tmp);
            }
        });

    let answer = stacks.map(|mut v| v.pop().unwrap());

    println!("{:?}", answer);
}

fn part2() {
    let mut stacks = [
        vec!["N", "S", "D", "C", "V", "Q", "T"],
        vec!["M", "F", "V"],
        vec!["F", "Q", "W", "D", "P", "N", "H", "M"],
        vec!["D", "Q", "R", "T", "F"],
        vec!["R", "F", "M", "N", "Q", "H", "V", "B"],
        vec!["C", "F", "G", "N", "P", "W", "Q"],
        vec!["W", "F", "R", "L", "C", "T"],
        vec!["T", "Z", "N", "S"],
        vec!["M", "S", "D", "J", "R", "Q", "H", "N"],
    ];

    include_str!("input.txt")
        .lines()
        .map(|s| {
            let (_, count, _, from, _, to) = s.split_whitespace().next_tuple().unwrap();
            (count.parse::<usize>().unwrap(), from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
        })
        .for_each(|(count, from, to)| {
            let mut q = vec![];
            for _ in 0..count {
                let tmp = stacks[from-1].pop().unwrap();
                q.push(tmp);
            }

            for _ in 0..count {
                stacks[to-1].push(q.pop().unwrap());
            }
        });

    let answer = stacks.map(|mut v| v.pop().unwrap());

    // Wrong: FRDSQRRCD
    println!("{:?}", answer);
}

fn main() {
    // part1();
    part2();
}
