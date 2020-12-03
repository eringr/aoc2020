
use std::fs::File;
use std::io::{BufReader, BufRead};

const WIDTH: usize = 31;

fn collect_input() -> Vec<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}

fn test_slope(inputs: &Vec<String>, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut pos = 0;
    for input in inputs.iter().skip(down).step_by(down) {
        pos += right;
        pos %= WIDTH;
        if input.chars().nth(pos).unwrap() == '#' {
            trees += 1;
        }
    }
    trees
}

fn solve_pt1(inputs: &Vec<String>) -> usize {
    return test_slope(inputs, 3, 1);
}

fn solve_pt2(inputs: &Vec<String>) -> usize {
    return
        test_slope(inputs, 1, 1) *
        test_slope(inputs, 3, 1) *
        test_slope(inputs, 5, 1) *
        test_slope(inputs, 7, 1) *
        test_slope(inputs, 1, 2);
}

fn main() {
    let inputs = collect_input();
    let answer_pt1 = solve_pt1(&inputs);
    let answer_pt2 = solve_pt2(&inputs);
    println!("Answer part 1: {}", answer_pt1);
    println!("Answer part 2: {}", answer_pt2);
}
