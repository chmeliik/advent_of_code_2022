use std::io::stdin;

use anyhow::Result;
use itertools::Itertools;

mod day1;
mod day2;
mod day3;

trait Solution<'a> {
    type ProblemRepr;
    type Output;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>;

    fn solve(input: &Self::ProblemRepr) -> Self::Output;
}

fn main() -> Result<()> {
    let lines: Vec<String> = stdin().lines().try_collect()?;

    let input = day3::Day3Part2::parse_input(lines.iter())?;
    let answer = day3::Day3Part2::solve(&input);

    println!("answer = {:?}", answer);

    Ok(())
}
