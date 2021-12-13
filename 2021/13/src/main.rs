use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut folds: Vec<(&str, i32)> = Vec::new();

    let mut fold = false;
    for line in &lines {
        let l = &line.len();
        if *l == 0 {
            fold = true;
            continue;
        }

        if fold {
            let parts: Vec<&str> = line.trim().split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect();
            folds.push((parts[0], parts[1].parse::<i32>().unwrap()));
        } else {
            let parts: Vec<i32> = line
                .trim()
                .split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect();

            points.push((parts[0], parts[1]));
        }
    }

    let mut points1 = points.clone();
    println!["part 1 = {}", part1(&mut points1, &folds)];
    let mut points2 = points.clone();
    println!["part 2 =\n"];
    part2(&mut points2, &folds);
}

fn part1(points: &mut Vec<(i32, i32)>, folds: &Vec<(&str, i32)>) -> usize {
    let f0 = folds[0];

    if f0.0 == "x" {
        for p in points.into_iter() {
            if p.0 >= f0.1 {
                let dx = p.0 - f0.1;
                p.0 = p.0 - 2 * dx;
            }
        }
    } else {
        for p in points.into_iter() {
            if p.1 >= f0.1 {
                let dy = p.1 - f0.1;
                p.1 = p.1 - 2 * dy;
            }
        }
    }

    let mut count: HashSet<String> = HashSet::new();
    for p in points {
        let key = format!["{} {}", p.0, p.1];
        count.insert(key);
    }
    count.len()
}

fn part2(points: &mut Vec<(i32, i32)>, folds: &Vec<(&str, i32)>) {
    for f in folds {
        if f.0 == "x" {
            for p in points.into_iter() {
                if p.0 >= f.1 {
                    let dx = p.0 - f.1;
                    p.0 = p.0 - 2 * dx;
                }
            }
        } else {
            for p in points.into_iter() {
                if p.1 >= f.1 {
                    let dy = p.1 - f.1;
                    p.1 = p.1 - 2 * dy;
                }
            }
        }
    }

    let mut max_y: i32 = 0;
    let mut max_x: i32 = 0;

    for p in points.into_iter() {
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }

    let mut grid = vec![Vec::new(); max_y as usize + 1];
    for y in 0..grid.len() {
        grid[y] = vec![false; max_x as usize + 1];
    }

    for p in points.into_iter() {
        grid[p.1 as usize][p.0 as usize] = true;
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    print!("\n");
}
