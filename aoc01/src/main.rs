fn main() {
    // part1();
    part2();
}

fn part1() {
    let answer = include_str!("input.txt")
        .lines()
        .fold([0, 0], |mut acc, s| {
            match s.parse::<i32>() {
                Ok(n) => {
                    acc[1] = n + acc[1];
                    acc
                },
                Err(_) => if acc[1] > acc[0] {
                    [acc[1], 0]
                } else {
                    acc[1] = 0;
                    acc
                }
            }
        });

    println!("{:?}", answer.iter().max());
}

fn part2() {
    let mut answer = include_str!("input.txt")
        .lines()
        .fold(vec![0], |mut acc, s| {
            match s.parse::<i32>() {
                Ok(n) => {
                    let i = acc.len() - 1;
                    acc[i] = n + acc[i];
                    acc
                },
                Err(_) => {
                    acc.push(0);
                    acc
                }
            }
        });
    answer.sort_by(|a, b| b.cmp(a));

    println!("{:?}", answer[0] + answer[1] + answer[2]);
}