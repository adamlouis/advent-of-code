fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");

    let mut values: Vec<i32> = Vec::new();
    for line in lines {
        let v = line.parse::<i32>().unwrap();
        values.push(v);
    }

    println!("part 1={}", part1(&values));
    println!("part 2={}", part2(&values));
}

fn part1(values: &Vec<i32>) -> u32 {
    let mut ret: u32 = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            ret += 1;
        }
    }
    ret
}

fn part2(values: &Vec<i32>) -> u32 {
    let mut agg: Vec<i32> = Vec::new();
    for i in 0..values.len() - 2 {
        agg.push(values[i] + values[i + 1] + values[i + 2])
    }
    part1(&agg)
}
