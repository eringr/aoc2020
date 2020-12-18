
use std::fs;
use std::collections::HashMap;

fn collect_inputs() -> Vec::<usize> {
    fs::read_to_string("input").expect("Unable to open input file")
        .split(",").map(|x| x.parse::<usize>().unwrap()).collect()
}

fn get_answer(inputs: &Vec::<usize>, last_turn: usize) -> usize {
    let mut turns_spoken = HashMap::<usize, usize>::new();
    for (turn, input) in inputs.iter().enumerate() {
        *turns_spoken.entry(*input).or_insert(0) = turn+1;
    }
    let mut slt = 0;
    let mut last_spoken = 0;
    for turn in inputs.len()..last_turn {
        last_spoken = if slt == 0 {0} else {turn-slt};
        let sle = turns_spoken.entry(last_spoken)
            .or_insert(0);
        slt = *sle;
        *sle = turn+1;
    }
    last_spoken
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", get_answer(&inputs, 2020));
    println!("Answer part 2: {:?}", get_answer(&inputs, 30000000));
}
