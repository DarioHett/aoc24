#[allow(dead_code)]
pub mod char_map_rep;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day15;
pub mod day16;

use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::io::BufRead;

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    grid, point
);
pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub enum Day02State {
    Initial,
    FirstStep(i32),
    Increasing(i32),
    Decreasing(i32),
    Unsafe,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
pub enum Dampened {
    Dampened,
    Undampened,
}
pub fn day02_is_safe(acc: Day02State, v: i32) -> Day02State {
    match acc {
        Day02State::Unsafe => Day02State::Unsafe,
        Day02State::Initial => return Day02State::FirstStep(v),
        Day02State::FirstStep(x) => match v - x {
            1 => Day02State::Increasing(v),
            2 => Day02State::Increasing(v),
            3 => Day02State::Increasing(v),
            -1 => Day02State::Decreasing(v),
            -2 => Day02State::Decreasing(v),
            -3 => Day02State::Decreasing(v),
            _ => Day02State::Unsafe,
        },
        Day02State::Increasing(x) => match v - x {
            1 => Day02State::Increasing(v),
            2 => Day02State::Increasing(v),
            3 => Day02State::Increasing(v),
            _ => Day02State::Unsafe,
        },
        Day02State::Decreasing(x) => match v - x {
            -1 => Day02State::Decreasing(v),
            -2 => Day02State::Decreasing(v),
            -3 => Day02State::Decreasing(v),
            _ => Day02State::Unsafe,
        },
    }
}

pub fn dampened_day02_is_safe(
    (accl, accr): (Day02State, Dampened),
    v: i32,
) -> (Day02State, Dampened) {
    let new_state = day02_is_safe(accl.clone(), v);
    match new_state {
        Day02State::Unsafe => match accr {
            Dampened::Dampened => (Day02State::Unsafe, accr),
            Dampened::Undampened => (accl, Dampened::Dampened),
        },
        _ => (new_state, accr),
    }
}

pub fn day05_parse_rule(line: String) -> (i32, i32) {
    let (l, r) = line.split_once("|").unwrap();
    (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
}

pub fn day05_parse_update(line: String) -> Vec<i32> {
    line.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn day05_parse<R: BufRead>(reader: R) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut parsing_rules = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let (k, v) = day05_parse_rule(line);
            rules.entry(k).or_insert(Vec::new()).push(v);
        } else {
            updates.push(day05_parse_update(line));
        }
    }
    (rules, updates)
}

pub fn day05_sort(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    // Rules: Key is to be printed (!) before any of the values.
    let mut sorted_update: Vec<i32> = Vec::new();
    sorted_update.push(update[0]);
    for v in update.into_iter().skip(1) {
        // Does `v` need to be printed before `w`?
        // => Yes, if `w` is in the values of (key) `v`.
        match rules.get(&v) {
            None => sorted_update.push(v.clone()),
            // There are no rules for `v`? => Can be last, push directly.
            Some(vals) => {
                let mut did_insert = false;
                for i in 0..sorted_update.len() {
                    if vals.contains(&sorted_update[i]) {
                        sorted_update.insert(i, v.clone());
                        did_insert = true;
                        break;
                    };
                    // No value in the sorted list needs to be printed before `v` => Insert last.
                }
                if !did_insert {
                    sorted_update.push(v.clone());
                }
            }
        }
    }
    sorted_update
}

pub fn day05_is_sorted(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    // Rules: Key is to be printed (!) before any of the values.
    let mut is_sorted: bool = true;
    for i in 0..update.len() {
        match rules.get(&update[i]) {
            // No rules associated w/ indexed value are to be skipped.
            None => continue,
            Some(vals) => {
                // Any value in `vals` has to appear after (!) the current index value.
                // <=> If we find a prior value in vals, the update is not sorted.
                for j in 0..(i + 1) {
                    if vals.contains(&update[j]) {
                        is_sorted = false;
                        break;
                    };
                }
            }
        }
    }
    is_sorted
}

pub fn day05_middle_number(sorted_update: &Vec<i32>) -> i32 {
    sorted_update[sorted_update.len().div_euclid(2)]
}

pub fn day06_orientation(c: char) -> (i32, i32) {
    match c {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("invalid orientation"),
    }
}

pub fn day06_is_guard(c: char) -> bool {
    match c {
        '^' => true,
        '>' => true,
        'v' => true,
        '<' => true,
        _ => false,
    }
}

pub fn day06_from_orientation(o: (i32, i32)) -> char {
    match o {
        (0, -1) => '^',
        (1, 0) => '>',
        (0, 1) => 'v',
        (-1, 0) => '<',
        _ => panic!("invalid orientation"),
    }
}

#[derive(Debug)]
pub struct Guard {
    pub pos: (i32, i32),
    pub dpos: (i32, i32),
}

impl Guard {
    pub fn new(pos: (i32, i32), dpos: (i32, i32)) -> Guard {
        Guard { pos, dpos }
    }

    pub fn print(&self) -> () {
        println!(
            "{2}: x={0},y={1}",
            self.pos.0,
            self.pos.1,
            day06_from_orientation(self.dpos)
        )
    }
    pub fn next_loc(&self) -> (i32, i32) {
        (self.pos.0 + self.dpos.0, self.pos.1 + self.dpos.1)
    }

    pub fn step(&self) -> Guard {
        Self::new(self.next_loc(), self.dpos)
    }

    pub fn rotate(&self) -> Guard {
        Guard {
            pos: self.pos,
            dpos: day06_rotate_clockwise(self.dpos),
        }
    }
}

pub fn day06_find_guard(a_map: &Vec<Vec<char>>) -> Option<Guard> {
    let nrows = a_map.len();
    let ncols = a_map[0].len();
    for (r, c) in iproduct!(0..nrows, 0..ncols) {
        if day06_is_guard(a_map[r][c]) {
            return Some(Guard::new(
                (c as i32, r as i32),
                day06_orientation(a_map[r][c]),
            ));
        }
    }
    None
}

pub fn day06_rotate_clockwise((x, y): (i32, i32)) -> (i32, i32) {
    (-y, x)
}

pub fn day07_can_be_made_true(test_value: &u64, evaluation_values: &Vec<u64>) -> bool {
    evaluation_values
        .into_iter()
        .map(|v| *v)
        .contains(test_value)
}

pub fn day07_parse_line(line: &str) -> (u64, Vec<u64>) {
    let (test_value, parameters) = line.split_once(": ").unwrap();
    (
        test_value.parse::<u64>().unwrap(),
        parameters
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
    )
}

pub fn day07_eval_operators_p2(parameters: &Vec<u64>, operators: &str) -> u64 {
    let mut v = parameters[0];
    for (i, c) in operators.chars().enumerate() {
        if c == '+' {
            v = &v + parameters[i + 1];
        } else if c == '*' {
            v = &v * parameters[i + 1];
        } else if c == '|' {
            let p = parameters[i + 1];
            v = v * (10u32.pow(p.ilog10() + 1) as u64) + p;
        }
    }
    v
}

pub fn day07_eval_operators(parameters: &Vec<u64>, operators: &str) -> u64 {
    let mut v = parameters[0];
    for (i, c) in operators.chars().enumerate() {
        if c == '+' {
            v = &v + parameters[i + 1];
        } else if c == '*' {
            v = &v * parameters[i + 1];
        }
    }
    v
}

pub fn day07_generate_operators_p2(n_ops: i32) -> Vec<String> {
    if n_ops < 0 {
        panic!("negative number of ops");
    }
    if n_ops == 0 {
        return Vec::new();
    }
    let ops_variables = vec!["*", "+", "|"];
    if n_ops == 1 {
        return ops_variables
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect();
    }
    let combinations: Vec<_> = (2..n_ops).fold(
        ops_variables
            .iter()
            .cartesian_product(ops_variables.iter())
            .map(|(&a, &b)| a.to_owned() + b)
            .collect(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(ops_variables.iter())
                .map(|(a, b)| a.to_owned() + b)
                .collect()
        },
    );
    combinations
}

pub fn day07_generate_operators_p2f() -> HashMap<i32, Vec<String>> {
    let mut map = HashMap::new();
    for i in 0..12 {
        map.insert(i, day07_generate_operators_p2(i));
    }
    map
}

pub fn day07_generate_operators(n_ops: i32) -> Vec<String> {
    if n_ops < 0 {
        panic!("negative number of ops");
    }
    if n_ops == 0 {
        return Vec::new();
    }
    let ops_variables = vec!["*", "+"];
    if n_ops == 1 {
        return ops_variables
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect();
    }
    let combinations: Vec<_> = (2..n_ops).fold(
        ops_variables
            .iter()
            .cartesian_product(ops_variables.iter())
            .map(|(&a, &b)| a.to_owned() + b)
            .collect(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(ops_variables.iter())
                .map(|(a, b)| a.to_owned() + b)
                .collect()
        },
    );
    combinations
}

pub fn day07_problem01(line: &str) -> (u64, Vec<String>, Vec<u64>) {
    let (tv, pms) = day07_parse_line(line);
    let op_combs = day07_generate_operators((pms.len() - 1) as i32);
    let possible_values = op_combs
        .iter()
        .cloned()
        .map(|c| day07_eval_operators(&pms, c.as_str()))
        .collect::<Vec<u64>>();
    (tv, op_combs, possible_values)
}

pub fn day07_problem02(line: &str) -> (u64, Vec<String>, Vec<u64>) {
    let (tv, pms) = day07_parse_line(line);
    let op_combs = day07_generate_operators_p2((pms.len() - 1) as i32);
    let possible_values = op_combs
        .iter()
        .cloned()
        .map(|c| day07_eval_operators_p2(&pms, c.as_str()))
        .collect::<Vec<u64>>();
    (tv, op_combs, possible_values)
}

pub fn day07_problem02_w_map(line: &str, map: &HashMap<i32, Vec<String>>) -> u64 {
    let (tv, pms) = day07_parse_line(line);
    let mut v = 0 as u64;
    for c in map.get(&((pms.len() - 1) as i32)).unwrap() {
        let w = day07_eval_operators_p2(&pms, c.as_str());
        if w == tv {
            v = w;
            break;
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }

    #[test]
    fn day05_is_sorted_test_1() {
        let update: Vec<i32> = vec![1, 2, 3];
        let mut rules = HashMap::new();
        rules.insert(2, vec![1]);
        let is_sorted = day05_is_sorted(&update, &rules);
        assert_eq!(false, is_sorted);
    }

    #[test]
    fn day05_is_sorted_test_2() {
        let update: Vec<i32> = vec![1, 2, 3];
        let mut rules = HashMap::new();
        rules.insert(2, vec![3]);
        let is_sorted = day05_is_sorted(&update, &rules);
        assert_eq!(true, is_sorted);
    }
    #[test]
    fn day05_sort_test_1() {
        let update: Vec<i32> = vec![1, 2];
        let mut rules = HashMap::new();
        rules.insert(2, vec![1]);
        let sorted_update = day05_sort(&update, &rules);
        assert_eq!(vec![2, 1], sorted_update);
    }

    #[test]
    fn day05_sort_test_2() {
        let update: Vec<i32> = vec![3, 1, 2];
        let mut rules = HashMap::new();
        rules.insert(2, vec![3]);
        rules.insert(3, vec![1]);
        let sorted_update = day05_sort(&update, &rules);
        assert_eq!(vec![2, 3, 1], sorted_update);
    }

    #[test]
    fn day05_sort_test_3() {
        let update: Vec<i32> = vec![61, 13, 29];
        let mut rules = HashMap::new();
        rules.insert(61, vec![13, 53, 29]);
        rules.insert(13, vec![]);
        rules.insert(29, vec![13]);
        let sorted_update = day05_sort(&update, &rules);
        assert_eq!(vec![61, 29, 13], sorted_update);
    }

    #[test]
    fn day07_parse_line_test() {
        let (t, v) = day07_parse_line("0: 1 2");
        assert_eq!(t, 0);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn day07_eval_operators_test_1() {
        let parameters = vec![1, 1, 1];
        let operators = "+*";
        let v = day07_eval_operators(&parameters, &operators);
        assert_eq!(v, 2);
    }

    #[test]
    fn day07_generate_operators_test_1() {
        let combinations = day07_generate_operators(3);
        println!("{:?}", combinations);
        assert_eq!(combinations.len(), 8);
    }
    #[test]
    fn day07_generate_operators_test_2() {
        let combinations = day07_generate_operators(2);
        println!("{:?}", combinations);
        assert_eq!(combinations.len(), 4);
    }
    #[test]
    fn day07_generate_operators_test_3() {
        let combinations = day07_generate_operators(1);
        println!("{:?}", combinations);
        assert_eq!(combinations.len(), 2);
    }

    #[test]
    fn day07_single_line_test() {
        let line = "190: 10 19";
        let (tv, pms) = day07_parse_line(line);
        let op_combs = day07_generate_operators((pms.len() - 1) as i32);
        let possible_values = op_combs
            .iter()
            .cloned()
            .map(|c| day07_eval_operators(&pms, c.as_str()))
            .collect::<Vec<u64>>();
        assert!(day07_can_be_made_true(&tv, &possible_values))
    }
}
