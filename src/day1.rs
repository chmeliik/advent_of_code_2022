use anyhow::{Context, Result};
use itertools::Itertools;

use crate::Solution;

pub struct Day1Part1;

impl<'a> Solution<'a> for Day1Part1 {
    type ProblemRepr = Vec<u64>;
    type Output = Option<u64>;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr, anyhow::Error>
    where
        I: Iterator<Item = &'a String>,
    {
        parse_to_vec64(lines)
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        input.iter().max().copied()
    }
}

pub struct Day1Part2;

impl<'a> Solution<'a> for Day1Part2 {
    type ProblemRepr = Vec<u64>;
    type Output = u64;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr, anyhow::Error>
    where
        I: Iterator<Item = &'a String>,
    {
        parse_to_vec64(lines)
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        let mut copy = input.clone();
        copy.sort();
        let top3 = &copy[copy.len() - 3..copy.len()];
        top3.iter().sum()
    }
}

fn parse_to_vec64<'a, I>(lines: I) -> Result<Vec<u64>, anyhow::Error>
where
    I: Iterator<Item = &'a String>,
{
    let groups = lines.group_by(|l| l.is_empty());
    groups
        .into_iter()
        .filter_map(|(is_empty, group)| if is_empty { None } else { Some(group) })
        .map(|group| {
            group
                .map(|line| {
                    line.parse::<u64>()
                        .with_context(|| format!("Failed to parse {} as u64", line))
                })
                .sum()
        })
        .try_collect()
}
