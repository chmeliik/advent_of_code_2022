mod letter {
    use anyhow::anyhow;

    pub struct Letter(char);

    impl Letter {
        pub fn index(&self) -> u32 {
            fn distance_from(from: char, to: char) -> u32 {
                to as u32 - from as u32
            }

            let c = self.0;
            match c {
                'a'..='z' => distance_from('a', c),
                'A'..='Z' => distance_from('A', c) + 26,
                _ => panic!("Letter({:?}) should not be possible", c),
            }
        }

        pub fn from_index(i: u32) -> Option<Letter> {
            fn shifted_from(c: char, offset: u32) -> Option<Letter> {
                char::from_u32(c as u32 + offset).map(Letter)
            }

            match i {
                0..=25 => shifted_from('a', i),
                26..=51 => shifted_from('A', i - 26),
                _ => None,
            }
        }
    }

    impl TryFrom<char> for Letter {
        type Error = anyhow::Error;

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                'a'..='z' | 'A'..='Z' => Ok(Letter(c)),
                _ => Err(anyhow!("not an ASCII letter: {}", c)),
            }
        }
    }

    impl From<Letter> for char {
        fn from(l: Letter) -> Self {
            l.0
        }
    }

    #[derive(Clone, Copy)]
    pub struct LetterSet(u64);

    impl LetterSet {
        fn empty() -> LetterSet {
            LetterSet(0)
        }

        fn singleton(l: Letter) -> LetterSet {
            LetterSet(1 << l.index())
        }

        pub fn union(&self, other: LetterSet) -> LetterSet {
            LetterSet(self.0 | other.0)
        }

        pub fn intersection(&self, other: LetterSet) -> LetterSet {
            LetterSet(self.0 & other.0)
        }
    }

    impl TryFrom<&str> for LetterSet {
        type Error = anyhow::Error;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            s.chars()
                .map(|c| c.try_into())
                .try_fold(LetterSet::empty(), |set, maybe_letter| {
                    maybe_letter.map(|l| set.union(LetterSet::singleton(l)))
                })
        }
    }

    impl IntoIterator for LetterSet {
        type Item = Letter;
        type IntoIter = IterLetters;

        fn into_iter(self) -> Self::IntoIter {
            IterLetters {
                set: self,
                curr_index: 0,
            }
        }
    }

    pub struct IterLetters {
        set: LetterSet,
        curr_index: u32,
    }

    impl Iterator for IterLetters {
        type Item = Letter;

        fn next(&mut self) -> Option<Self::Item> {
            for i in self.curr_index..52 {
                if self.set.0 & (1 << i) != 0 {
                    self.curr_index = i + 1;
                    return Letter::from_index(i);
                }
            }
            self.curr_index = 52;
            None
        }
    }
}

use anyhow::Result;
use itertools::Itertools;

use crate::Solution;
use letter::LetterSet;

pub struct Day3Part1;

impl<'a> Solution<'a> for Day3Part1 {
    type ProblemRepr = Vec<(LetterSet, LetterSet)>;
    type Output = u64;

    fn parse_input<I>(lines: I) -> Result<Self::ProblemRepr>
    where
        I: Iterator<Item = &'a String>,
    {
        fn two_letter_sets(line: &String) -> Result<(LetterSet, LetterSet)> {
            let (a, b) = line.split_at(line.len() / 2);
            Ok((a.try_into()?, b.try_into()?))
        }

        lines.map(two_letter_sets).try_collect()
    }

    fn solve(input: &Self::ProblemRepr) -> Self::Output {
        input.iter().fold(0, |sum, (set1, set2)| {
            let s: u32 = set1
                .intersection(*set2)
                .into_iter()
                .map(|letter| letter.index() + 1)
                .sum();
            sum + s as u64
        })
    }
}

#[cfg(test)]
mod letter_tests {
    use super::letter::Letter;
    use super::letter::LetterSet;

    #[test]
    fn letter() {
        let l = Letter::from_index(26).unwrap();
        assert_eq!(l.index(), 26);
    }

    #[test]
    fn letter_set() {
        let s1 = LetterSet::try_from("abcdef").unwrap();
        let s2 = LetterSet::try_from("ABCDEF").unwrap();
        let s3 = s1.union(s2);
        let letters: Vec<char> = s3.into_iter().map(|l| l.into()).collect();
        assert_eq!(
            letters,
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F']
        )
    }
}
