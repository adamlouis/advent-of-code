use crate::aoc;
use crate::Day;

pub struct Day01 {}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Day for Day01 {
    fn num(&self) -> u8 {
        1
    }
    fn input1(&self) -> String {
        return aoc::read("input/01_input.txt");
    }
    fn input2(&self) -> String {
        return aoc::read("input/01_input.txt");
    }
    fn part1(&self, input: &String) -> String {
        format!("{}", part1(input.lines()))
    }
    fn part2(&self, input: &String) -> String {
        format!("{}", part2(input.lines()))
    }
}

fn part1(lines: std::str::Lines) -> u32 {
    lines.map(|l| get_num(l)).sum()
}

fn part2(lines: std::str::Lines) -> u32 {
    lines.map(|l| get_num_word(l)).sum()
}

fn get_num(line: &str) -> u32 {
    let first: u32 = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first * 10 + last
}

fn get_num_word(line: &str) -> u32 {
    let line_string = line.to_string();

    let num_at = move |idx: usize| -> Option<u32> {
        let target = &line_string[idx..];

        if let Some(v) = target.chars().nth(0)?.to_digit(10) {
            return Some(v);
        }

        let word_idx = NUMS
            .iter()
            .enumerate()
            .find(|(_, &w)| target.starts_with(w))
            .and_then(|(i, _)| Some(i as u32 + 1));

        if word_idx.is_some() {
            return word_idx;
        }
        return None;
    };

    let first = (0..line.len()).find_map(|idx| num_at(idx)).unwrap();
    let last = (0..line.len()).rev().find_map(|idx| num_at(idx)).unwrap();

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_example_1() {
        let d = Day01 {};
        assert_eq!(d.part1(&EXAMPLE1.to_string()), "142");
    }

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_example_2() {
        let d = Day01 {};
        assert_eq!(d.part2(&EXAMPLE2.to_string()), "281");
    }
}
