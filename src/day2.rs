use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

use crate::Solution;

mod rps {
    #[derive(PartialEq, Clone, Copy)]
    pub enum RPS {
        Rock,
        Paper,
        Scissors,
    }

    use RPS::*;

    impl RPS {
        pub fn beats(&self) -> RPS {
            Self::from_index((*self as i8 - 1).rem_euclid(3))
        }

        pub fn beaten_by(&self) -> RPS {
            Self::from_index((*self as i8 + 1).rem_euclid(3))
        }

        fn from_index(i: i8) -> Self {
            match i {
                0 => Rock,
                1 => Paper,
                2 => Scissors,
                _ => panic!("RPS::from_index({}) is invalid", i),
            }
        }
    }
}

use self::rps::{RPS, RPS::*};

pub struct Day2Part1;

impl<'a> Solution<'a> for Day2Part1 {
    type ProblemRepr = Vec<(RPS, RPS)>;
    type Output = u64;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>,
    {
        fn word_to_rps(w: &str) -> Result<RPS> {
            match w {
                "A" | "X" => Ok(Rock),
                "B" | "Y" => Ok(Paper),
                "C" | "Z" => Ok(Scissors),
                _ => Err(anyhow!("not a recognized symbol: {}", w)),
            }
        }

        fn two_rps(line: &String) -> Result<(RPS, RPS)> {
            let mut words = line.split(char::is_whitespace);
            let w1 = words.next().ok_or(anyhow!("no words in: {}", line))?;
            let w2 = words.next().ok_or(anyhow!("only one word in: {}", line))?;
            if words.next().is_some() {
                return Err(anyhow!("more than two words in: {}", line));
            }
            Ok((
                word_to_rps(w1).with_context(|| format!("in line: {}", line))?,
                word_to_rps(w2).with_context(|| format!("in line: {}", line))?,
            ))
        }

        lines.map(two_rps).try_collect()
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        fn shape_score(rps: &RPS) -> u64 {
            match rps {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            }
        }

        input.iter().fold(0, |acc, (opponent, me)| {
            let points_for_round = shape_score(me)
                + if &me.beats() == opponent {
                    6
                } else if me == opponent {
                    3
                } else {
                    0
                };
            points_for_round + acc
        })
    }
}

pub struct Day2Part2;

pub enum Outcome {
    Lose,
    Draw,
    Win,
}

use Outcome::*;

impl<'a> Solution<'a> for Day2Part2 {
    type ProblemRepr = Vec<(RPS, Outcome)>;
    type Output = u64;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>,
    {
        fn convert_to_outcome(rps: &RPS) -> Outcome {
            match rps {
                Rock => Lose,
                Paper => Draw,
                Scissors => Win,
            }
        }

        let v = Day2Part1::parse_input(lines)?
            .iter()
            .map(|(opponent, me)| (*opponent, convert_to_outcome(me)))
            .collect();
        Ok(v)
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        fn convert_to_rps(outcome: &Outcome, opponent: &RPS) -> RPS {
            match outcome {
                Lose => opponent.beats(),
                Draw => *opponent,
                Win => opponent.beaten_by(),
            }
        }

        let problem1_repr = input
            .iter()
            .map(|(opponent, outcome)| (*opponent, convert_to_rps(outcome, opponent)))
            .collect();

        Day2Part1::solve(&problem1_repr)
    }
}
