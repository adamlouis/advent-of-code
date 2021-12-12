use std::collections::HashMap;
use std::collections::HashSet;

struct Sequence {
    input: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split("\n").collect();
    let mut seqs: Vec<Sequence> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split("|").collect();
        let input: Vec<String> = sortstr(&parts[0].trim().split(" ").collect());
        let output: Vec<String> = sortstr(&parts[1].trim().split(" ").collect());
        seqs.push(Sequence {
            input: input,
            output: output,
        });
    }

    println!["{}", part1(&seqs)];
    println!["{}", part2(&seqs)];
}

// sort chars in string
fn sortstr(ss: &Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for &s in ss {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        ret.push(chars.into_iter().collect());
    }
    ret
}

fn part1(seqs: &Vec<Sequence>) -> i32 {
    let mut count = 0;
    for seq in seqs {
        for o in &seq.output {
            let l = o.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                count += 1;
            }
        }
    }
    count
}

fn part2(seqs: &Vec<Sequence>) -> i32 {
    let mut total = 0;
    let nums = vec![1, 4, 7, 8, 0, 2, 6, 9, 3, 5];
    for seq in seqs {
        let mut mapping: HashMap<String, i32> = HashMap::new();
        let mut remaining: Vec<&String> = seq.input.iter().map(|s| s).collect();
        let mut next: Vec<&String>;
        let mut one = String::from("");

        // fought w/ borrow checker to not copy strings a ton
        // need to figure out best practices
        while remaining.len() > 0 {
            // pick next value
            let v = pick(&remaining, &one);

            // get mapping
            let n = nums[mapping.len()];

            // remember 1 to help disambiguate later sequences :/
            if n == 1 {
                one = v.to_string();
            }

            // update mapping
            // copies str to help w/ borrow checker
            mapping.insert(v.to_string(), n);

            // filter out for next iteration
            // borrow checker w/ filter, etc?
            // do in < O(N)
            // etc
            next = Vec::new();
            for &r in &remaining {
                if !r.eq(v) {
                    next.push(r);
                }
            }
            remaining = next;
        }

        // decode output from the mapping we've deduced
        let mut v = 0;
        let mut unit = 1;
        for o in seq.output.iter().rev() {
            let d = mapping.get(o).unwrap();
            v += d * unit;
            unit = unit * 10;
        }
        total += v;
    }

    total
}

// 1 - 2 - __c__f_ # has len 2
// 4 - 4 - _bcd_f_ # has len 4
// 7 - 3 - a_c__f_ # has len 3
// 8 - 7 - abcdefg # has len 7
// 0 - 6 - abc_efg # missing letter (d) & len 6
// 2 - 5 - a_cde_g # missing letter (f)
// 6 - 6 - ab_defg # has uniq letter (e)
// 9 - 6 - abcd_fg # has len 6
// 3 - 5 - a_cd_fg # contains the number 1
// 5 - 5 - ab_d_fg # does not contain the number 1

fn pick<'a>(vs: &'a Vec<&String>, one: &String) -> &'a String {
    match vs.len() {
        10 => return with_len(&vs, 2),
        9 => return with_len(&vs, 4),
        8 => return with_len(&vs, 3),
        7 => return with_len(&vs, 7),
        6 => {
            let res = without_uniq(vs);
            if res.len() != 2 {
                panic!("ah");
            }
            if res[0].len() == 6 {
                return &res[0];
            }
            return &res[1];
        }
        5 => return without_uniq(vs)[0],
        4 => return with_uniq(vs)[0],
        3 => return with_len(&vs, 6),
        2 => {
            if contains_chars(&vs[0], one) {
                return &vs[0];
            }
            return &vs[1];
        }
        1 => return &vs[0],
        _ => {
            unreachable!()
        }
    }
}

fn contains_chars(src: &String, target: &String) -> bool {
    let mut chars = HashSet::new();

    for c in src.chars() {
        chars.insert(c);
    }

    for c in target.chars() {
        if !chars.contains(&c) {
            return false;
        }
    }

    return true;
}

fn with_len<'a>(vs: &Vec<&'a String>, n: usize) -> &'a String {
    return vs.iter().find(|&s| s.len() == n).unwrap();
}

fn with_uniq<'a>(vs: &Vec<&'a String>) -> Vec<&'a String> {
    let mut words_by_char: HashMap<char, Vec<&String>> = HashMap::new();

    for s in vs {
        for c in s.chars() {
            if words_by_char.get(&c) == None {
                words_by_char.insert(c, Vec::new());
            }
            let v: &mut Vec<&String> = words_by_char.get_mut(&c).unwrap(); //.unwrap();
            v.push(s);
        }
    }

    let mut ret: Vec<&String> = Vec::new();
    for (_, v) in words_by_char {
        if v.len() == 1 {
            ret.push(&v[0]);
        }
    }
    ret
}

fn without_uniq<'a>(vs: &Vec<&'a String>) -> Vec<&'a String> {
    let mut words_by_char: HashMap<char, Vec<String>> = HashMap::new();

    for s in vs {
        for c in s.chars() {
            if words_by_char.get(&c) == None {
                words_by_char.insert(c, Vec::new());
            }
            let v: &mut Vec<String> = words_by_char.get_mut(&c).unwrap(); //.unwrap();
            v.push(s.to_string());
        }
    }

    let mut ret: Vec<&String> = Vec::new();

    for (_, v) in words_by_char {
        if v.len() == vs.len() - 1 {
            for target in vs {
                if !v.contains(&target.to_string()) {
                    ret.push(&target);
                }
            }
        }
    }

    ret
}
