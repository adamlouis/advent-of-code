use crate::aoc;
use crate::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn num(&self) -> u8 {
        2
    }
    fn input1(&self) -> String {
        return aoc::read("input/02_input.txt");
    }
    fn input2(&self) -> String {
        return aoc::read("input/02_input.txt");
    }
    fn part1(&self, input: &String) -> String {
        format!("{}", part1(input.lines()))
    }
    fn part2(&self, input: &String) -> String {
        format!("{}", part2(input.lines()))
    }
}

fn part1(lines: std::str::Lines) -> u32 {
    lines.map(|l| get_id_if_ok(l)).sum()
}

fn part2(lines: std::str::Lines) -> u32 {
    lines.map(|l| get_power(l)).sum()
}

fn get_id_if_ok(line: &str) -> u32 {
    let line_string = line.to_string();

    let colon_idx = line.find(':').unwrap();

    let mut color = String::from("");
    let mut count_str = String::from("");

    let mut find_count = true;

    for (i, c) in line_string[colon_idx + 2..].chars().enumerate() {
        let last = (line_string.len() - 1 - colon_idx - 2) == i;
        if last {
            color.push(c);
        }

        if c == ' ' || c == ',' || c == ';' || last {
            if color.len() > 0 && count_str.len() > 0 {
                let count = count_str.parse::<u32>().unwrap();
                match color.as_str() {
                    "red" => {
                        if count > 12 {
                            return 0;
                        }
                    }
                    "green" => {
                        if count > 13 {
                            return 0;
                        }
                    }
                    "blue" => {
                        if count > 14 {
                            return 0;
                        }
                    }
                    _ => {}
                }

                count_str = String::from("");
                color = String::from("");
                find_count = true;
            }
            if count_str.len() > 0 {
                find_count = false;
            }
        } else {
            if find_count {
                count_str.push(c);
            } else {
                color.push(c);
            }
        }
    }

    line_string[5..colon_idx].parse::<u32>().unwrap()
}

fn get_power(line: &str) -> u32 {
    let line_string = line.to_string();

    let colon_idx = line.find(':').unwrap();

    let mut color = String::from("");
    let mut count_str = String::from("");

    let mut find_count = true;

    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;

    for (i, c) in line_string[colon_idx + 2..].chars().enumerate() {
        let last = (line_string.len() - 1 - colon_idx - 2) == i;
        if last {
            color.push(c);
        }

        if c == ' ' || c == ',' || c == ';' || last {
            if color.len() > 0 && count_str.len() > 0 {
                let count = count_str.parse::<u32>().unwrap();
                match color.as_str() {
                    "red" => {
                        max_r = std::cmp::max(max_r, count);
                    }
                    "green" => {
                        max_g = std::cmp::max(max_g, count);
                    }
                    "blue" => {
                        max_b = std::cmp::max(max_b, count);
                    }
                    _ => {}
                }

                count_str = String::from("");
                color = String::from("");
                find_count = true;
            }
            if count_str.len() > 0 {
                find_count = false;
            }
        } else {
            if find_count {
                count_str.push(c);
            } else {
                color.push(c);
            }
        }
    }

    max_r * max_g * max_b
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_example_1() {
        let d = Day02 {};
        assert_eq!(d.part1(&EXAMPLE.to_string()), "8");
    }

    #[test]
    fn test_example_2() {
        let d = Day02 {};
        assert_eq!(d.part2(&EXAMPLE.to_string()), "2286");
    }
}
