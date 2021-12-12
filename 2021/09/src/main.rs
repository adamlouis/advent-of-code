use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut input: Vec<Vec<i32>> = Vec::new();

    for line in &lines {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push((c as u32 - '0' as u32) as i32);
        }
        input.push(row);
    }

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input));
}

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let lows = get_lows(input);
    for low in &lows {
        ret += input[low.0 as usize][low.1 as usize] + 1;
    }
    ret
}

fn get_lows(input: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let l = get(&input, &(i as i32, j as i32 - 1));
            let r = get(&input, &(i as i32, j as i32 + 1));
            let u = get(&input, &(i as i32 - 1, j as i32));
            let d = get(&input, &(i as i32 + 1, j as i32));
            let v = input[i][j];
            if v < l && v < r && v < u && v < d {
                ret.push((i as i32, j as i32));
            }
        }
    }
    ret
}

fn part2(input: &Vec<Vec<i32>>) -> usize {
    let lows = get_lows(input);

    let mut sizes: Vec<usize> = Vec::new();

    for low in &lows {
        let mut seen: HashSet<String> = HashSet::new();
        let mut current: Vec<(i32, i32)> = vec![low.clone()];
        let mut found: Vec<(i32, i32)> = Vec::new();

        while current.len() > 0 {
            let mut next = vec![];
            for pair in current {
                if !in_bounds(&input, &pair) {
                    continue;
                }

                if get(&input, &pair) == 9 {
                    continue;
                }

                let key: String = format!["{}_{}", pair.0, pair.1];

                if seen.contains(&key) {
                    continue;
                }

                found.push(pair);
                seen.insert(key);

                next.push((pair.0 + 1, pair.1));
                next.push((pair.0 - 1, pair.1));
                next.push((pair.0, pair.1 + 1));
                next.push((pair.0, pair.1 - 1));
            }
            current = next;
        }
        sizes.push(found.len());
    }
    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn in_bounds(input: &Vec<Vec<i32>>, pair: &(i32, i32)) -> bool {
    return !(pair.0 < 0
        || pair.1 < 0
        || pair.0 >= input.len() as i32
        || pair.1 >= input[pair.0 as usize].len() as i32);
}

fn get(input: &Vec<Vec<i32>>, pair: &(i32, i32)) -> i32 {
    if !in_bounds(input, pair) {
        return 9;
    }
    return input[pair.0 as usize][pair.1 as usize];
}
