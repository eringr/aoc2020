
use std::fs;
use std::io::{BufReader, BufRead};

fn collect_inputs() -> Vec<Vec<char>> {
    let f = fs::File::open("input").expect("Unable to open input file");
    BufReader::new(f).lines()
        .map(|x| x.unwrap().chars().filter(|&x| x != ' ').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn handle_precedence_pt1(out: &mut Vec::<char>, ops: &mut Vec::<char>) {
    while let c @ Some('+') | c @ Some('*') = ops.last() {
        out.push(*c.unwrap());
        ops.pop();
    }
}

fn handle_precedence_pt2(out: &mut Vec::<char>, ops: &mut Vec::<char>) {
    while let Some('+') = ops.last() {
        out.push('+');
        ops.pop();
    }
}

fn get_result(
    tokens: &Vec::<char>,
    do_precedence: &dyn Fn(&mut Vec::<char>, &mut Vec::<char>),
) -> u64 {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();
    for token in tokens {
        match *token {
            n @ '0'..='9' => output.push(n),
            c @ '+' | c @ '*' => {
                do_precedence(&mut output, &mut operator_stack);
                operator_stack.push(c);
            },
            c @ '(' => operator_stack.push(c),
            ')' => {
                loop {
                    match operator_stack.pop().unwrap() {
                        '(' => break,
                        c => output.push(c),
                    };
                };
            },
            t => panic!("Unexpected token {}", t),
        };
    }
    output.extend(operator_stack.iter().rev());

    let mut operands = Vec::new();
    for c in output {
        match c {
            '+' => {
                let (l, r) = (operands.pop().unwrap(), operands.pop().unwrap());
                operands.push(l + r);
            },
            '*' => {
                let (l, r) = (operands.pop().unwrap(), operands.pop().unwrap());
                operands.push(l * r);
            },
            n @ '0'..='9' => operands.push(n.to_digit(10).unwrap() as u64),
            t => panic!("Unexpected token {}", t),
        }
    }
    operands.pop().unwrap()
}

fn main() {
    let inputs = collect_inputs();
    let (answer_pt1, answer_pt2): (u64, u64) = (
        inputs.iter().map(|x| get_result(x, &handle_precedence_pt1)).sum(),
        inputs.iter().map(|x| get_result(x, &handle_precedence_pt2)).sum(),
    );
    println!("Answer part 1: {:?}", answer_pt1);
    println!("Answer part 2: {:?}", answer_pt2);
}
