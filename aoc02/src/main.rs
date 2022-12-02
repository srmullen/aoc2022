#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw
}

use Hand::*;
use Outcome::*;

fn get_hand(thrown: &str) -> Hand {
    match thrown {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissor,
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissor,
        _ => panic!(),
    }
}

fn calc_score(opp: Hand, me: Hand) -> i32 {
    let hand = match me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3,
    };

    let win = if opp == me {
        3
    } else if (me == Hand::Rock && opp == Hand::Scissor) || (me == Hand::Paper && opp == Hand::Rock) || (me == Hand::Scissor && opp == Hand::Paper) {
        6
    } else {
        0
    };

    println!("hand: {}, win: {}", hand, win);

    hand + win
}

fn get_hand_by_outcome(hand: Hand, outcome: Outcome) -> Hand {
    match outcome {
        Draw => hand,
        Loss => match hand {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        },
        Win => match hand {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        }
    }
}

fn part1() {
    let answer = include_str!("input.txt")
        .lines()
        .fold(0, |total, round| {
            let (opp, me) = round.split_at(1);
            let me = me.trim();
            let score = calc_score(get_hand(opp), get_hand(me));
            println!("Round: {}, score: {}", round, score);
            total + score
        });
    
    println!("Answer: {}", answer);
}

fn part2() {
    let answer = include_str!("input.txt")
        .lines()
        .fold(0, |total, round| {
            let (opp, strat) = round.split_at(1);
            let opp = get_hand(opp);
            let strat = strat.trim();
            let outcome = match strat {
                "X" => Loss,
                "Y" => Draw,
                "Z" => Win,
                _ => panic!(),
            };
            let me = get_hand_by_outcome(opp.clone(), outcome);
           total + calc_score(opp, me)
        });
    println!("Answer: {}", answer);
}

fn main() {
    // part1();
    part2();
}