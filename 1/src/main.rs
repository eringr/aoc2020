
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

const TARGET: i64 = 2020;

fn collect_input() -> HashSet<i64> {
    let mut ret = HashSet::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.insert(line.expect("").parse::<i64>().expect(""));
    }
    ret
}

fn solve_part_1(inputs: &HashSet<i64>) -> Option<i64> {
    for input in inputs.iter() {
        let complement = TARGET - input;
        if inputs.contains(&complement) {
            return Some(input * complement);
        }
    }
    None
}

fn solve_part_2(inputs: &HashSet<i64>) -> Option<i64> {
    // caching based solution would be better O(), but favoring code clarity
    for combo in inputs.iter().combinations(3) {
        if let TARGET = combo.iter().map(|&x| x).sum() {
            return Some(combo.iter().map(|&x| x).product());
        }
    }
    None
}

fn main() {
    let inputs = collect_input();
    let parts = [solve_part_1, solve_part_2];
    for (n, solve_func) in parts.iter().enumerate() {
        if let Some(answer) = solve_func(&inputs) {
            println!("Part {} solution: {}", n+1, answer);
        } else {
            println!("Part {} solution not found", n+1);
        }
    }
}
