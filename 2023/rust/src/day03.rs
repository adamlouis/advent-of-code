use crate::aoc;
use crate::Day;

pub struct Day03 {}

impl Day for Day03 {
    fn num(&self) -> u8 {
        3
    }
    fn input1(&self) -> String {
        return aoc::read("input/03_input.txt");
    }
    fn input2(&self) -> String {
        return aoc::read("input/03_input.txt");
    }
    fn part1(&self, input: &String) -> String {
        format!("{}", part1(input.lines()))
    }
    fn part2(&self, input: &String) -> String {
        format!("{}", part2(input.lines()))
    }
}

#[derive(Debug)]
struct Num {
    val: u64,
    i: usize,
    j: usize,
    len: usize,
}

#[derive(Debug)]
struct Symbol {
    val: char,
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct Board {
    nums: Vec<Num>,
    syms: Vec<Symbol>,
}

fn parse_board(lines: std::str::Lines) -> Board {
    let mut ret = Board {
        nums: Vec::new(),
        syms: Vec::new(),
    };

    for (i, line) in lines.enumerate() {
        let mut num_start: Option<usize> = None;
        let mut num_current: u64 = 0;
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if num_start.is_none() {
                    num_start = Some(j);
                }
                num_current = num_current * 10 + c.to_digit(10).unwrap() as u64;
            }

            let last = j == line.len() - 1;

            if !c.is_digit(10) || last {
                if let Some(num_start_v) = num_start {
                    let mut len = j - num_start_v;
                    if last {
                        len += 1;
                    }
                    ret.nums.push(Num {
                        val: num_current,
                        i: i,
                        j: num_start_v,
                        len: len,
                    })
                }

                num_start = None;
                num_current = 0;
            }

            if !c.is_digit(10) && c != '.' {
                ret.syms.push(Symbol { val: c, i: i, j: j })
            }
        }
    }
    ret
}

fn are_adjacent(num: &Num, sym: &Symbol) -> bool {
    let di: u64 = (num.i as i32 - sym.i as i32).abs() as u64;

    let dj: u64 = (|| {
        if sym.j >= num.j && sym.j < num.j + num.len {
            return 0;
        }

        let dj_start = (sym.j as i32 - num.j as i32).abs();
        let dj_end = (sym.j as i32 - ((num.j + num.len - 1) as i32)).abs();

        return std::cmp::min(dj_start, dj_end) as u64;
    })();

    return di <= 1 && dj <= 1;
}

fn part1(lines: std::str::Lines) -> u64 {
    let board = parse_board(lines);

    let mut ret: u64 = 0;
    for num in board.nums.iter() {
        let syms: Vec<&Symbol> = board
            .syms
            .iter()
            .filter(|sym| are_adjacent(num, sym))
            .collect();

        if syms.len() > 0 {
            ret += num.val;
        }
    }

    ret
}

fn part2(lines: std::str::Lines) -> u64 {
    let board = parse_board(lines);

    let mut ret: u64 = 0;
    for sym in board.syms.iter() {
        if sym.val != '*' {
            continue;
        }

        let nums: Vec<&Num> = board
            .nums
            .iter()
            .filter(|num| are_adjacent(num, sym))
            .collect();

        if nums.len() == 2 {
            ret += nums[0].val * nums[1].val;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_example_1() {
        let d = Day03 {};
        assert_eq!(d.part1(&EXAMPLE.to_string()), "4361");
    }

    #[test]
    fn test_example_2() {
        let d = Day03 {};
        assert_eq!(d.part2(&EXAMPLE.to_string()), "467835");
    }
}
