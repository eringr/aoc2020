
use std::fs;
use itertools::Itertools;

fn collect_input() -> Vec<String> {
    let mut ret = Vec::new();
    let f = fs::read_to_string("input").expect("Unable to open input file");
    for line in f.split("\r\n\r\n") {
        ret.push(line.to_string());
    }
    ret
}

fn solve_pt1(inputs: &Vec<String>) -> usize {
    let mut count = 0;
    for input in inputs {
        for _ in input.replace("\r\n","").chars().unique() {
            count += 1;
        }
    }
    count
}

fn solve_pt2(inputs: &Vec<String>) -> usize {
    let mut count = 0;
    for input in inputs {
        let lines = input.split("\r\n").count();
        let mut last_c = '\0';
        let mut char_n = 0;
        for c in input.chars().sorted() {
            if c == last_c {
                char_n += 1;
            } else {
                char_n = 1;
            }
            if char_n == lines {
                count += 1;
            }
            last_c = c;
        }
    }
    count
}

fn main() {
    let inputs = collect_input();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
