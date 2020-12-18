
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::convert::TryFrom;
use std::cmp::Ordering;

type Floorplan = Vec::<Vec::<char>>;

fn collect_inputs() -> Floorplan {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.unwrap().chars().collect());
    }
    ret
}

fn get_tile(floorplan: &Floorplan, xi: i64, yi: i64) -> Option<&char> {
    if let (Some(xn), Some(yn)) = (
        usize::try_from(xi).ok(),
        usize::try_from(yi).ok()
    ) {
        if let Some(row) = floorplan.get(yn) {
            return row.get(xn);
        }
    }
    None
}

fn is_occupied(floorplan: &Floorplan, xi: i64, yi: i64) -> bool {
    match get_tile(floorplan, xi, yi) {
        Some('#') => true,
        _ => false,
    }
}

fn new_tile(floorplan: &Floorplan, xi: usize, yi: usize) -> char {
    let xn = i64::try_from(xi).unwrap();
    let yn = i64::try_from(yi).unwrap();
    let current_tile = *get_tile(floorplan, xn, yn).unwrap();
    if '.' == current_tile {
        return '.';
    }

    let x_search = [xn-1, xn, xn+1];
    let y_search = [yn-1, yn, yn+1];
    let surrounding = x_search.iter().cartesian_product(&y_search)
        .filter(|&(x, y)| (*x, *y) != (xn, yn))
        .filter(|&(x, y)| is_occupied(floorplan, *x, *y))
        .count();
    match surrounding {
        n if n >= 4 => 'L',
        n if n == 0 => '#',
        _ => current_tile,
    }
}

fn do_step(floorplan: &Floorplan) -> Floorplan {
    let mut floorplan_new = floorplan.clone();
    for (y, line) in floorplan_new.iter_mut().enumerate() {
        for (x, tile) in line.iter_mut().enumerate() {
            *tile = new_tile(floorplan, x, y);
        }
    }
    floorplan_new
}

fn count_occupied(floorplan: &Floorplan) -> usize {
    floorplan.iter().fold(0,
        |n, i| n + i.iter().filter(|&x| *x == '#').count()
    )
}

fn solve_pt1(floorplan: &Floorplan) -> usize {
    let mut floorplan_last = floorplan.clone();
    loop {
        let floorplan_new = do_step(&floorplan_last);
        if let Ordering::Equal = floorplan_new.cmp(&floorplan_last) {
            break count_occupied(&floorplan_new)
        }
        floorplan_last = floorplan_new;
    }
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    // let (answer_pt1, answer_pt2) = solve_both(&inputs);
}

// fn print_floorplan(floorplan: &Floorplan) {
//     for line in floorplan {
//         for c in line {
//             print!("{}", c);
//         }
//         println!("");
//     }
// }
