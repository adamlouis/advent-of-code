// https://adventofcode.com/2021/day/3
fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    // indexing into a string in rust requires iterator because chars are utf-8 & we need to compute how many bytes in each
    // for this problem, we can assume all chars in ascii
    // https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
    let lines: Vec<&[u8]> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.as_bytes())
        .collect();

    println!("part 1={}", part1(&lines));
    println!("part 2={}", part2(&lines));
}

fn part1(lines: &Vec<&[u8]>) -> u32 {
    let mut ones: Vec<u32> = vec![0; lines[0].len()];

    // count 1s at each index
    for line in lines {
        for (j, c) in line.iter().enumerate() {
            if *c == b'1' {
                ones[j] += 1;
            }
        }
    }

    let mut result = String::from("");

    for v in ones {
        // add 1 if more than half are ones ... equality case is undefined in docs
        if v > (lines.len() as u32 / 2) {
            result += "1"
        } else {
            result += "0"
        }
    }

    let gamma: u32 = frombin(result.as_str());
    let extra = 32 - lines[0].len();
    let epsilon: u32 = (gamma ^ (!0)) << extra >> extra;
    gamma * epsilon
}

fn part2(lines: &Vec<&[u8]>) -> u32 {
    filter_lines(lines, true) * filter_lines(lines, false)
}

fn frombin(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut ret: u32 = 0;
    let mut mask: u32 = 1;
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'1' {
            ret = ret | mask;
        }
        mask = mask << 1;
    }
    ret
}

fn filter_lines(lines: &Vec<&[u8]>, most_common: bool) -> u32 {
    let mut current: Vec<&[u8]> = lines.clone();
    let mut next: Vec<&[u8]> = Vec::new();
    let mut index = 0;

    loop {
        let sz = current.len();
        if sz == 0 {
            panic!("no strings remaining");
        }
        if sz == 1 {
            // success
            return frombin(std::str::from_utf8(current[0]).unwrap());
        }
        if index > current[0].len() {
            panic!("multiple string remaining but index out of bounds");
        }

        // count bits at nth position
        let mut count = 0;
        for line in &current {
            if line[index] == b'1' {
                count += 1;
            }
        }

        let keep_1s: bool;
        if most_common {
            // if tied, keep 1s
            keep_1s = (count > sz / 2) || (sz % 2 == 0 && count == sz / 2);
        } else {
            // if tied, keep 0s
            keep_1s = (count < sz / 2) || (sz % 2 == 1 && count == sz / 2);
        }

        for line in &current {
            let c = line[index];
            if c == b'1' && keep_1s {
                next.push(line);
            } else if c == b'0' && !keep_1s {
                next.push(line);
            }
        }

        index += 1;
        current = next;
        next = Vec::new();
    }
    // unreachable
}
