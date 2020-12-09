
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn collect_inputs() -> Vec::<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}


fn solve_pt1(mem: &Vec::<String>) -> i64 {
    let (acc, _) = run_with_swap(mem, None);
    acc
}

fn solve_pt2(mem: &Vec::<String>) -> i64 {
    for i in 1.. {
        // println!("running with swap_at = {}", i);
        match run_with_swap(mem, Some(i)) {
            (acc, true) => return acc,
            _ => (),
        }
    }
    0
}

fn run_with_swap(
    mem: &Vec::<String>,
    swap_at: Option<usize>
) -> (i64, bool) {
    let mut c = Computer {
        acc: 0,
        pc: 0,
    };
    let mut swap_skip = 0;
    let mut visited = HashSet::<usize>::new();
    for _ in 0.. {
        loop {
            if visited.contains(&c.pc) {
                return (c.acc, false);
            }
            if c.pc >= mem.len() {
                return (c.acc, true)
            }
            visited.insert(c.pc);
            if let Some(inst) = mem.get(c.pc) {
                let new_inst = match swap_at {
                    Some(swap_at) if swap_at == swap_skip => {
                        match &inst[0..3] {
                            "nop" => inst.replace("nop", "jmp"),
                            "jmp" => inst.replace("jmp", "nop"),
                            _ => inst.to_string(),
                        }
                    },
                    _ => inst.to_string(),
                };
                do_instruction(&mut c, new_inst);
                match &inst[0..3] {
                    "nop"|"jmp" => swap_skip += 1,
                    _ => (),
                };
            } else {
                panic!("fetching outside mem");
            }
        }
    }
    (0, false)
}

fn do_instruction(c: &mut Computer, inst: String) {
    // println!("{:<9} encountered, pc: {}", inst, c.pc);
    match &inst[0..3] {
        "nop" => {
            c.pc += 1;
        },
        "acc" => {
            c.acc += inst[4..].parse::<i64>().unwrap();
            c.pc += 1;
        },
        "jmp" => {
            let new_pc = (c.pc as i64) + inst[4..].parse::<i64>().unwrap();
            c.pc = new_pc as usize;
        },
        _ => {},
    }
}

pub struct Computer {
    acc: i64,
    pc: usize,
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
