use core::panic;

struct Step {
    direction: String,
    size: i32,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");

    let mut steps: Vec<Step> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        steps.push(Step {
            direction: parts[0].to_string(),
            size: parts[1].parse::<i32>().unwrap(),
        })
    }

    println!("part 1={}", part1(&steps));
    println!("part 2={}", part2(&steps));
}

fn part1(steps: &Vec<Step>) -> i32 {
    let (mut h, mut d): (i32, i32) = (0, 0);
    for step in steps.iter() {
        match step.direction.as_str() {
            "forward" => {
                h += step.size;
            }
            "down" => {
                d += step.size;
            }
            "up" => {
                d -= step.size;
            }
            _ => {
                panic!("unexpected direction: {}", step.direction);
            }
        }
    }
    h * d
}

fn part2(steps: &Vec<Step>) -> i32 {
    let (mut h, mut d, mut a): (i32, i32, i32) = (0, 0, 0);
    for step in steps.iter() {
        match step.direction.as_str() {
            "forward" => {
                h += step.size;
                d += a * step.size;
            }
            "down" => {
                a += step.size;
            }
            "up" => {
                a -= step.size;
            }
            _ => {
                panic!("unexpected direction: {}", step.direction);
            }
        }
    }
    h * d
}
