fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut values: Vec<i32> = lines[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!["part 1 = {:?}", part1(&mut values)];
    println!["part 2 = {:?}", part2(&mut values)];
}

fn part1(values: &mut Vec<i32>) -> i32 {
    values.sort();
    let median: i32 = if values.len() % 2 == 1 {
        values[values.len() / 2]
    } else {
        if values[values.len() / 2] != values[values.len() / 2 - 1] {
            panic!("ah!");
        }
        values[values.len() / 2]
    };

    let mut fuel: i32 = 0;
    for v in (&values).iter() {
        let f = (*v - median).abs();
        fuel += f;
    }
    fuel
}

fn part2(values: &mut Vec<i32>) -> i32 {
    let mut meanf: f64 = 0.0;
    for v in (&values).iter() {
        meanf += (*v as f64) / (values.len() as f64);
    }
    let mean = meanf.floor() as i32;
    min(get_fuel(values, mean), get_fuel(values, mean + 1))
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }
    return b;
}

fn get_fuel(values: &Vec<i32>, target: i32) -> i32 {
    let mut fuel: i32 = 0;
    for v in (&values).iter() {
        let n = (target - *v).abs();
        fuel += get_fuel_n(n);
    }
    fuel
}

fn get_fuel_n(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n % 2 == 0 {
        return (n + 1) * (n / 2);
    }
    return get_fuel_n(n - 1) + n;
}
