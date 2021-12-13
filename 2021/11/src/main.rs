fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut nums: Vec<Vec<u32>> = vec![];
    for line in &lines {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            row.push(c as u32 - '0' as u32);
        }
        nums.push(row);
    }

    let mut part1: u32 = 0;
    let part2;

    let mut i: u32 = 0;
    loop {
        let v = step(&mut nums);
        if i < 100 {
            part1 += v;
        }
        if v == 100 {
            part2 = i + 1;
            break;
        }
        i += 1;
    }

    println!["part1 {}", part1];
    println!["part2 {}", part2];
}

fn step(nums: &mut Vec<Vec<u32>>) -> u32 {
    let mut count: u32 = 0;

    let mut flashes: Vec<(i32, i32)> = vec![];
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            nums[i][j] += 1;
            if nums[i][j] > 9 {
                count += 1;
                flashes.push((i as i32, j as i32));
            }
        }
    }

    while flashes.len() > 0 {
        let mut next = vec![];
        for flash in flashes {
            let neighbors = vec![
                (flash.0 + 1, flash.1),
                (flash.0 - 1, flash.1),
                (flash.0, flash.1 + 1),
                (flash.0, flash.1 - 1),
                (flash.0 + 1, flash.1 + 1),
                (flash.0 + 1, flash.1 - 1),
                (flash.0 - 1, flash.1 + 1),
                (flash.0 - 1, flash.1 - 1),
            ];
            for n in neighbors {
                if in_bounds(&nums, &n) {
                    let v = nums[n.0 as usize][n.1 as usize];
                    if v < 9 {
                        nums[n.0 as usize][n.1 as usize] += 1;
                    } else if v == 9 {
                        nums[n.0 as usize][n.1 as usize] += 1;
                        next.push(n);
                        count += 1;
                    }
                }
            }
        }
        flashes = next;
    }
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            if nums[i][j] == 10 {
                nums[i][j] = 0;
            }
        }
    }
    count
}

fn in_bounds(nums: &Vec<Vec<u32>>, pair: &(i32, i32)) -> bool {
    return pair.0 >= 0
        && pair.1 >= 0
        && (pair.0 as usize) < nums.len()
        && (pair.1 as usize) < nums[pair.0 as usize].len();
}
