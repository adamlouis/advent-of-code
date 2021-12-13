use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.trim().split("-").collect();
        let n1 = parts[0];
        let n2 = parts[1];

        if !nodes.contains_key(n1) {
            nodes.insert(n1, Vec::new());
        }
        if !nodes.contains_key(n2) {
            nodes.insert(n2, Vec::new());
        }

        nodes.get_mut(n1).unwrap().push(n2);
        nodes.get_mut(n2).unwrap().push(n1);
    }

    println!["part 1 = {}", part1(&nodes)];
    println!["part 2 = {}", part2(&nodes)];
}

fn part1(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut done: Vec<Vec<&str>> = vec![];
    let mut paths: Vec<Vec<&str>> = vec![];
    paths.push(vec!["start"]);
    while paths.len() > 0 {
        let mut next: Vec<Vec<&str>> = vec![];
        for p in paths {
            let last = p[p.len() - 1];
            for conn in &nodes[last] {
                let mut step = p.clone();
                step.push(conn);

                if *conn == "end" {
                    done.push(step);
                } else if valid1(&step) {
                    next.push(step);
                }
            }
        }
        paths = next;
    }
    return done.len();
}

fn valid1(path: &Vec<&str>) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();

    for &p in path {
        let c0 = p.chars().nth(0).unwrap();
        let small = c0 >= 'a' && c0 <= 'z';
        if !small {
            continue;
        }

        if seen.contains(p) {
            return false;
        }
        seen.insert(p);
    }

    return true;
}

fn part2(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut done: Vec<Vec<&str>> = vec![];
    let mut paths: Vec<Vec<&str>> = vec![];
    paths.push(vec!["start"]);
    while paths.len() > 0 {
        let mut next: Vec<Vec<&str>> = vec![];
        for p in paths {
            let last = p[p.len() - 1];
            for conn in &nodes[last] {
                let mut step = p.clone();
                step.push(conn);

                if *conn == "end" {
                    done.push(step);
                } else if valid2(&step) {
                    next.push(step);
                }
            }
        }
        paths = next;
    }
    return done.len();
}

fn valid2(path: &Vec<&str>) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut extra = false;
    for &p in path {
        let c0 = p.chars().nth(0).unwrap();
        let small = c0 >= 'a' && c0 <= 'z';
        if !small {
            continue;
        }

        if seen.contains(p) {
            if extra || p == "start" || p == "end" {
                return false;
            }
            extra = true;
        }
        seen.insert(p);
    }

    return true;
}
