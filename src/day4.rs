mod section_range {
    use std::ops::RangeInclusive;

    #[derive(PartialEq)]
    pub struct SectionRange(RangeInclusive<u32>);

    impl SectionRange {
        pub fn new(range: RangeInclusive<u32>) -> SectionRange {
            SectionRange(range)
        }

        fn intersection(&self, other: &SectionRange) -> SectionRange {
            let a = &self.0;
            let b = &other.0;

            let start: u32 = *a.start().max(b.start());
            let end: u32 = *a.end().min(b.end());

            SectionRange(start..=end)
        }

        pub fn fully_contains(&self, other: &SectionRange) -> bool {
            &self.intersection(other) == other
        }

        pub fn any_overlap(&self, other: &SectionRange) -> bool {
            !self.intersection(other).0.is_empty()
        }
    }
}

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::Solution;

use self::section_range::SectionRange;

pub struct Day4Part1;

impl<'a> Solution<'a> for Day4Part1 {
    type ProblemRepr = Vec<(SectionRange, SectionRange)>;
    type Output = usize;

    fn parse_input<I>(lines: I) -> anyhow::Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>,
    {
        fn parse_line(line: &String) -> Result<(SectionRange, SectionRange)> {
            let parse_range = |range_str: &str| -> Result<SectionRange> {
                let nums: Vec<u32> = range_str.split('-').map(|s| s.parse()).try_collect()?;
                let (a, b) = nums.iter().collect_tuple().ok_or(anyhow!(
                    "invalid range: {:?} (in line: {:?})",
                    range_str,
                    line
                ))?;
                Ok(SectionRange::new((*a)..=(*b)))
            };

            let (a, b) = line
                .split(',')
                .collect_tuple()
                .ok_or(anyhow!("invalid range pair: {:?}", line))?;

            Ok((parse_range(a)?, parse_range(b)?))
        }

        lines.map(parse_line).try_collect()
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        input
            .iter()
            .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
            .count()
    }
}

pub struct Day4Part2;

impl<'a> Solution<'a> for Day4Part2 {
    type ProblemRepr = Vec<(SectionRange, SectionRange)>;
    type Output = usize;

    fn parse_input<I>(lines: I) -> anyhow::Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>,
    {
        Day4Part1::parse_input(lines)
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        input.iter().filter(|(a, b)| a.any_overlap(b)).count()
    }
}
