
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn collect_inputs() -> Vec::<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}

struct MemState {
    mem: HashMap<usize, usize>,
    mask_set: usize,
    mask_clear: usize,
    dcare_bits: Vec::<usize>,
}

fn process_mask(mem_state: &mut MemState, input: &str) {
    mem_state.mask_set = 0;
    mem_state.mask_clear = 0;
    mem_state.dcare_bits.clear();
    for (i, c) in input.chars().rev().enumerate() {
        match c {
            '1' => mem_state.mask_set |= 1<<i,
            '0' => mem_state.mask_clear |= 1<<i,
            'X' => mem_state.dcare_bits.push(1<<i),
            _ => panic!("unexpected input"),
        };
    }
}

fn process_mem_pt1(mem_state: &mut MemState, addr: usize, val: usize) {
    let mut new_val = val;
    new_val |= mem_state.mask_set;
    new_val &= !mem_state.mask_clear;
    mem_state.mem.insert(addr, new_val);
}

fn process_mem_pt2(mem_state: &mut MemState, addr: usize, val: usize) {
    let mut new_addr = addr;
    new_addr |= mem_state.mask_set;
    new_addr &= !(mem_state.dcare_bits.iter().sum::<usize>());

    mem_state.mem.insert(new_addr, val);
    for i in 1..=(mem_state.dcare_bits.len()) {
        for combo in mem_state.dcare_bits.iter().combinations(i) {
            let mask: usize = combo.iter().map(|&x| x).sum();
            mem_state.mem.insert(new_addr | mask, val);
        }
    }
}

fn get_answer(
    inputs: &Vec::<String>,
    mem_fn: &dyn Fn(&mut MemState, usize, usize),
) -> usize {
    let mut mem_state = MemState {
        mem: HashMap::new(),
        mask_set: 0,
        mask_clear: 0,
        dcare_bits: Vec::new(),
    };
    let mem_match = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap();
    let mask_match = Regex::new(r"mask = ([01X]+)").unwrap();

    for input in inputs {
        if let Some(m) = mem_match.captures(input) {
            mem_fn(
                &mut mem_state,
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
        } else if let Some(m) = mask_match.captures(input) {
            process_mask(&mut mem_state, m.get(1).unwrap().as_str());
        }
    }

    mem_state.mem.values().sum()
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", get_answer(&inputs, &process_mem_pt1));
    println!("Answer part 2: {:?}", get_answer(&inputs, &process_mem_pt2));
}
