fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let drawings: Vec<u32> = to_ints(lines[0], ",");

    // create list of boards for part 1 & part 2
    let size = 5;
    let mut boards1: Vec<Board> = Vec::new();
    let mut boards2: Vec<Board> = Vec::new();
    let mut values: Vec<Vec<u32>> = Vec::new();
    for i in 2..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }
        values.push(to_ints(lines[i], " "));
        if values.len() == size {
            boards1.push(Board::new(values.clone()));
            boards2.push(Board::new(values));
            values = Vec::new();
        }
    }

    println!("part 1={}", part1(boards1, &drawings));
    println!("part 2={}", part2(boards2, &drawings));
}

fn part1(mut boards: Vec<Board>, drawings: &Vec<u32>) -> u32 {
    for d in drawings {
        for b in &mut boards {
            b.mark(*d);
            if b.done() {
                return d * b.score();
            }
        }
    }
    panic!("no solution")
}

fn part2(mut boards: Vec<Board>, drawings: &Vec<u32>) -> u32 {
    let mut current: Vec<&mut Board> = Vec::new();
    for b in &mut boards {
        current.push(b);
    }

    for d in drawings {
        let mut next: Vec<&mut Board> = Vec::new();
        let count = current.len();
        for c in current {
            c.mark(*d);

            if !c.done() {
                next.push(c);
            } else {
                if count == 1 {
                    return d * c.score();
                }
            }
        }
        current = next;
    }
    panic!("no solution")
}

struct Board {
    values: Vec<Vec<u32>>,
    seen: Vec<Vec<bool>>,
}

impl Board {
    fn new(values: Vec<Vec<u32>>) -> Self {
        let seen: Vec<Vec<bool>> = vec![vec![false; values.len()]; values.len()];
        Self {
            values: values,
            seen: seen,
        }
    }
    // slow but fine
    fn mark(&mut self, target: u32) {
        for (i, row) in self.values.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if target == *v {
                    self.seen[i][j] = true;
                }
            }
        }
    }
    // slow but fine
    fn done(&self) -> bool {
        for i in 0..self.seen.len() {
            let mut all = true;
            for j in 0..self.seen[0].len() {
                if !self.seen[i][j] {
                    all = false;
                    break;
                }
            }
            if all {
                return true;
            }
        }
        for j in 0..self.seen[0].len() {
            let mut all = true;
            for i in 0..self.seen.len() {
                if !self.seen[i][j] {
                    all = false;
                    break;
                }
            }
            if all {
                return true;
            }
        }

        false
    }
    fn score(&self) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..self.seen.len() {
            for j in 0..self.seen[0].len() {
                if !self.seen[i][j] {
                    ret += self.values[i][j];
                }
            }
        }
        ret
    }
}

// do i need all of the collect / iters ?
fn to_ints(s: &str, d: &str) -> Vec<u32> {
    clean(s.trim())
        .as_str()
        .split(d)
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

// remove depulicate spaces
// write manually so i can learn rust syntax
fn clean(s: &str) -> String {
    let mut ret = String::new();
    let mut prev: Option<char> = None;
    for (_, c) in s.chars().enumerate() {
        if prev == None {
            if c != ' ' {
                ret = ret + &c.to_string();
            }
        } else {
            if c != ' ' {
                ret = ret + &c.to_string();
            } else {
                if prev != Some(' ') {
                    ret = ret + &c.to_string();
                }
            }
        }
        prev = Some(c);
    }
    ret
}
