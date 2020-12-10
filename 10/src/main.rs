
use std::fs::File;
use std::io::{BufReader, BufRead};

fn collect_inputs() -> Vec::<i64> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.unwrap().parse::<i64>().unwrap());
    }
    ret.sort();
    ret
}

fn solve_both(inputs: &Vec::<i64>) -> (i64, i64) {
    let get_combinations = |x: i64| {
        let mut ret = 1i64;
        if x > 1 {
            ret = 2i64.pow(x as u32 - 1);
        }
        if x > 3 {
            ret -= 2i64.pow(x as u32 - 4);
        }
        ret
    };
    let (mut last_i, mut jump_one, mut jump_three) = (0,0,0);
    let mut jump_one_seq = 0;
    let mut combinations = 1;
    for i in inputs {
        if i - last_i == 1 {
            jump_one += 1;
            jump_one_seq += 1;
        } else if i - last_i == 3 {
            jump_three += 1;
            combinations *= get_combinations(jump_one_seq);
            jump_one_seq = 0;
        } else {
            panic!("unexpected");
        }
        last_i = *i;
    }
    combinations *= get_combinations(jump_one_seq);
    (jump_one * (jump_three+1), combinations)
}

fn main() {
    let inputs = collect_inputs();
    let (answer_pt1, answer_pt2) = solve_both(&inputs);
    println!("Answer part 1: {:?}", answer_pt1);
    println!("Answer part 2: {:?}", answer_pt2);
}
