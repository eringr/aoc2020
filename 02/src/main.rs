
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn collect_input() -> Vec<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}

fn solve_both(inputs: &Vec<String>) -> (usize, usize) {
    let pattern = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    let mut valid_pt1 = 0usize;
    let mut valid_pt2 = 0usize;
    for input in inputs {
        let cs = pattern.captures(input).expect("Unexpected input");
        let (from, to, c, target) = (
            cs.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            cs.get(3).unwrap().as_str().chars().next().unwrap(),
            cs.get(4).unwrap().as_str()
        );

        let count = target.chars().filter(|x| *x == c).count();
        if count >= from && count <= to {
            valid_pt1 += 1;
        }

        if let (Some(first), Some(second)) = (
            target.chars().nth(from-1),
            target.chars().nth(to-1)
        ) {
            if (c == first) != (c == second) {
                valid_pt2 += 1;
            }
        }
    }
    (valid_pt1, valid_pt2)
}

fn main() {
    let inputs = collect_input();
    let (answer_pt1, answer_pt2) = solve_both(&inputs);
    println!("Answer part 1: {}", answer_pt1);
    println!("Answer part 2: {}", answer_pt2);
}
