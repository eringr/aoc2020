
use std::fs;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

type Space = HashMap<Vec<i64>, bool>;
const SIM_RANGE: std::ops::Range<i64> = -6i64..13i64;

fn collect_inputs(dim: usize) -> Space {
    let f = fs::File::open("input").expect("Unable to open input file");
    let inputs = BufReader::new(f).lines()
        .map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut ret = Space::new();
    for (y, row) in inputs.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            let mut inserter = Vec::<i64>::new();
            inserter.push(x as i64);
            inserter.push(y as i64);
            for _ in 0..dim-2 {
                inserter.push(0);
            }
            ret.insert(inserter, *ch == '#');
        }
    }
    return ret;
}

fn count_active(space: &Space) -> usize {
    let dims = space.keys().next().unwrap().len();
    let count_points = vec![0; dims].iter().map(|_| SIM_RANGE)
        .multi_cartesian_product()
        .collect::<Vec<_>>();
    count_points.iter()
        .filter(|&x| *space.get(x).unwrap_or(&false))
        .count()
}

fn get_surrounding(space: &Space, point: &Vec<i64>) -> (bool, usize) {
    let point_active = *space.get(point).unwrap_or(&false);
    let test_points = point.iter().map(|i| i-1..=i+1)
        .multi_cartesian_product()
        .filter(|x| x != point)
        .collect::<Vec<_>>();
    let count = test_points.iter()
        .filter(|&p| *space.get(p).unwrap_or(&false))
        .count();
    (point_active, count)
}

fn do_step(space: &Space) -> Space {
    let mut new_space = space.clone();
    let dims = space.keys().next().unwrap().len();
    let sim_points = vec![0; dims].iter().map(|_| SIM_RANGE)
        .multi_cartesian_product()
        .collect::<Vec<_>>();
    for point in &sim_points {
        let active = new_space.entry(point.to_vec()).or_insert(false);
        *active = match get_surrounding(space, point) {
            (true, _c @ 2..=3) => true,
            (true, _) => false,
            (false, _c @ 3) => true,
            _ => false,
        };
    }
    new_space
}

fn get_answer(space: &Space) -> usize {
    let mut last_space = space.clone();
    for _ in 0..6 {
        last_space = do_step(&last_space);
    }
    count_active(&last_space)
}


fn main() {
    let space_pt1 = collect_inputs(3);
    println!("Answer part 1: {:?}", get_answer(&space_pt1));
    let space_pt2 = collect_inputs(4);
    println!("Answer part 2: {:?}", get_answer(&space_pt2));
}
