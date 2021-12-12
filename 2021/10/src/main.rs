fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut tot = 0;
    for line in &lines {
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                let top: char = stack[stack.len() - 1];
                if matches(top, c) {
                    stack.remove(stack.len() - 1);
                } else {
                    tot += points(c);
                    break;
                }
            }
        }
    }
    println!["part1 {}", tot];

    let mut scores: Vec<u64> = Vec::new();
    for line in &lines {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                let top: char = stack[stack.len() - 1];
                if matches(top, c) {
                    stack.remove(stack.len() - 1);
                } else {
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            let mut score: u64 = 0;
            for c in stack.iter().rev() {
                score = score * 5 + points(*c) as u64;
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!["part2 {}", scores[scores.len() / 2]];
}

fn is_open(c: char) -> bool {
    return c == '(' || c == '<' || c == '[' || c == '{';
}

fn matches(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '{' => close == '}',
        '[' => close == ']',
        '<' => close == '>',
        _ => unreachable!(),
    }
}

fn points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        // part 2
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
