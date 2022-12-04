use itertools::Itertools;

fn range_contains(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    // a is contained by b, b is contained by a
    a1 >= b1 && a2 <= b2 || b1 >= a1 && b2 <= a2
}

fn range_overlaps(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1
}

fn part1() {
    let answer = include_str!("input.txt")
        .lines()
        .map(|s| s.split(",").next_tuple().unwrap())
        .map(|(r1, r2)| {
            let (a1, a2) = r1.split("-").next_tuple().unwrap();
            let (b1, b2) = r2.split("-").next_tuple().unwrap();
            if range_contains(
                a1.parse::<i32>().unwrap(), 
                a2.parse::<i32>().unwrap(), 
                b1.parse::<i32>().unwrap(), 
                b2.parse::<i32>().unwrap()
            ) { 1 } else { 0 }
        }).reduce(|a, b| a + b);

    println!("{:?}", answer);
}

fn part2() {
    let answer = include_str!("input.txt")
        .lines()
        .map(|s| s.split(",").next_tuple().unwrap())
        .map(|(r1, r2)| {
            let (a1, a2) = r1.split("-").next_tuple().unwrap();
            let (b1, b2) = r2.split("-").next_tuple().unwrap();
            if range_overlaps(
                a1.parse::<i32>().unwrap(), 
                a2.parse::<i32>().unwrap(), 
                b1.parse::<i32>().unwrap(), 
                b2.parse::<i32>().unwrap()
            ) { 1 } else { 0 }
        }).reduce(|a, b| a + b);

    println!("{:?}", answer);
}

fn main() {
    // part1();
    part2();
}
