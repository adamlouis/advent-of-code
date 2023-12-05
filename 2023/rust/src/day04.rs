use crate::aoc;
use crate::Day;
use std::collections::HashSet;

pub struct Day04 {}

impl Day for Day04 {
    fn num(&self) -> u8 {
        4
    }
    fn input1(&self) -> String {
        return aoc::read("input/04_input.txt");
    }
    fn input2(&self) -> String {
        return aoc::read("input/04_input.txt");
    }
    fn part1(&self, input: &String) -> String {
        format!("{}", part1(input.lines()))
    }
    fn part2(&self, input: &String) -> String {
        format!("{}", part2(input.lines()))
    }
}
fn part1(lines: std::str::Lines) -> u32 {
    lines
        .map(|l| {
            let (s, _) = get_score(l);
            s
        })
        .sum()
}

fn part2(line_iter: std::str::Lines) -> u32 {
    let lines: Vec<&str> = line_iter.collect();

    let mut counts: Vec<u32> = vec![0; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        // 1 by default
        counts[i] += 1;

        let (_, c) = get_score(line);
        let inc = counts[i];

        for j in (i + 1)..(i + 1 + c as usize) {
            if j < counts.len() {
                counts[j as usize] += inc;
            }
        }
    }
    counts.iter().sum()
}

fn get_score(line: &str) -> (u32, u32) {
    let colon_idx = line.find(':').unwrap();
    let target_str = &line.to_string()[colon_idx + 1..];

    let mut start: Option<usize> = None;
    let mut seen_pipe = false;

    let mut winning: HashSet<u32> = HashSet::new();
    let mut yours: HashSet<u32> = HashSet::new();

    for (i, c) in target_str.chars().enumerate() {
        let is_last = (target_str.len() - 1) == i;

        if c.is_digit(10) {
            if start.is_none() {
                start = Some(i);
            }
        }

        if start.is_some() && (!c.is_digit(10) || is_last) {
            let end = if is_last { i + 1 } else { i };
            let v = target_str[start.unwrap()..end].parse::<u32>().unwrap();
            if seen_pipe {
                winning.insert(v);
            } else {
                yours.insert(v);
            }
            start = None;
        }

        if c == '|' {
            seen_pipe = true;
        }
    }

    let mut score = 0;
    let mut count = 0;
    for y in yours {
        if winning.contains(&y) {
            count += 1;
            if score == 0 {
                score = 1;
            } else {
                score = score * 2;
            }
        }
    }
    (score, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_example_1() {
        let d = Day04 {};
        assert_eq!(d.part1(&EXAMPLE.to_string()), "13");
    }

    #[test]
    fn test_example_2() {
        let d = Day04 {};
        assert_eq!(d.part2(&EXAMPLE.to_string()), "30");
    }
}
