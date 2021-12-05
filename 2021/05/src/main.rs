struct Segment {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    // parse to struct
    let mut segments: Vec<Segment> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let s: Vec<u32> = parts[0]
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let e: Vec<u32> = parts[1]
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        segments.push(Segment {
            x1: s[0],
            y1: s[1],
            x2: e[0],
            y2: e[1],
        })
    }

    // solve
    println!("part 1={}", part1(&segments));
    println!("part 2={}", part2(&segments));
}

fn part1(segments: &Vec<Segment>) -> u32 {
    let mut ret: u32 = 0;
    let mut seen = std::collections::HashMap::<String, u32>::new();
    for seg in segments {
        if seg.x1 == seg.x2 {
            let range = get_range(seg.y1, seg.y2);
            for y in range {
                let k = key(seg.x1, y);
                let v: u32 = match seen.get(&k) {
                    None => 0,
                    Some(i) => *i,
                };
                match v {
                    0 => {
                        seen.insert(k, 1);
                    }
                    1 => {
                        seen.insert(k, v + 1);
                        ret += 1;
                    }
                    _ => {
                        seen.insert(k, v + 1);
                    }
                }
            }
        } else if seg.y1 == seg.y2 {
            let range = get_range(seg.x1, seg.x2);
            for x in range {
                let k = key(x, seg.y1);
                let v: u32 = match seen.get(&k) {
                    None => 0,
                    Some(i) => *i,
                };
                match v {
                    0 => {
                        seen.insert(k, 1);
                    }
                    1 => {
                        seen.insert(k, v + 1);
                        ret += 1;
                    }
                    _ => {
                        seen.insert(k, v + 1);
                    }
                }
            }
        }
    }

    ret
}

fn part2(segments: &Vec<Segment>) -> u32 {
    let mut ret: u32 = 0;
    let mut seen = std::collections::HashMap::<String, u32>::new();
    for seg in segments {
        // lots of copy / paste ¯\_(ツ)_/¯
        if seg.x1 == seg.x2 {
            let range = get_range(seg.y1, seg.y2);
            for y in range {
                let k = key(seg.x1, y);
                let v: u32 = match seen.get(&k) {
                    None => 0,
                    Some(i) => *i,
                };

                match v {
                    0 => {
                        seen.insert(k, 1);
                    }
                    1 => {
                        seen.insert(k, v + 1);
                        ret += 1;
                    }
                    _ => {
                        seen.insert(k, v + 1);
                    }
                }
            }
        } else if seg.y1 == seg.y2 {
            let range = get_range(seg.x1, seg.x2);
            for x in range {
                let k = key(x, seg.y1);
                let v: u32 = match seen.get(&k) {
                    None => 0,
                    Some(i) => *i,
                };

                match v {
                    0 => {
                        seen.insert(k, 1);
                    }
                    1 => {
                        seen.insert(k, v + 1);
                        ret += 1;
                    }
                    _ => {
                        seen.insert(k, v + 1);
                    }
                }
            }
        } else {
            let mut rangex = get_range(seg.x1, seg.x2);
            let mut rangey = get_range(seg.y1, seg.y2);
            loop {
                let x = match &rangex.next() {
                    None => break,
                    Some(i) => *i,
                };
                let y = match &rangey.next() {
                    None => break,
                    Some(i) => *i,
                };

                let k = key(x, y);
                let v: u32 = match seen.get(&k) {
                    None => 0,
                    Some(i) => *i,
                };

                match v {
                    0 => {
                        seen.insert(k, 1);
                    }
                    1 => {
                        seen.insert(k, v + 1);
                        ret += 1;
                    }
                    _ => {
                        seen.insert(k, v + 1);
                    }
                }
            }
        }
    }
    ret
}

fn key(x: u32, y: u32) -> String {
    format!["{} {}", x, y]
}

// return a increasing or decreasing range from x to y, inclusive
fn get_range(x: u32, y: u32) -> Box<dyn Iterator<Item = u32>> {
    if x < y {
        return Box::new(x..y + 1);
    }
    Box::new((y..x + 1).rev())
}
