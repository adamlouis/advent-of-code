use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut values: Vec<u32> = lines[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // consider counts of each value rather than each value individually
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for k in &values {
        let v = match counts.get(&k) {
            None => 0,
            Some(i) => *i,
        };
        counts.insert(*k, v + 1);
    }

    println!["part 1 = {:?}", part1(&mut values, 80)];
    println!["part 2 = {:?}", part2(&mut counts, 256)];
}

// slow version
fn part1(values: &mut Vec<u32>, n: u32) -> usize {
    if n == 0 {
        return values.len();
    }
    for i in 0..values.len() {
        if values[i] == 0 {
            values[i] = 6;
            values.push(8);
        } else {
            values[i] -= 1;
        }
    }
    return part1(values, n - 1);
}

// fast version
fn part2(counts: &mut HashMap<u32, usize>, n: u32) -> usize {
    if n == 0 {
        let mut sum: usize = 0;
        for (_, v) in counts {
            sum = sum + *v;
        }
        return sum;
    }

    let new = match counts.get(&0) {
        None => 0,
        Some(i) => *i,
    };

    for i in 0..8 {
        let prev = match counts.get(&(i + 1)) {
            None => 0,
            Some(i) => *i,
        };
        counts.insert(i, prev);
    }

    let d6 = match counts.get(&6) {
        None => 0,
        Some(i) => *i,
    };

    counts.insert(8, new);
    counts.insert(6, d6 + new);

    return part2(counts, n - 1);
}
