use std::time::Instant;

use aoc;

fn main() {
    let days: Vec<Box<dyn aoc::Day>> = vec![
        Box::new(aoc::Day01 {}),
        Box::new(aoc::Day02 {}),
        Box::new(aoc::Day03 {}),
    ];

    println!("----------------------------------------------------------------");
    for d in days {
        println!("day {}", d.num());
        println!("");
        let input1 = d.input1();
        let input2 = d.input2();
        let start = Instant::now();
        let p1 = d.part1(&input1);
        let end_p1 = Instant::now();
        let p2 = d.part2(&input2);
        let end_p2 = Instant::now();
        println!("part 1 = {} ({:?})", p1, end_p1.duration_since(start));
        println!("part 2 = {} ({:?})", p2, end_p2.duration_since(end_p1));
        println!("----------------------------------------------------------------");
    }
}
