use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use aoc24::day11::*;

const DAY: &str = "11";
const INPUT_FILE: &str = "input/11.txt";

const TEST: &str = "125 17";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut v = parse_input(reader.lines().map(|l| l.unwrap()).next().unwrap().as_str());
        for i in 0..25 {
            println!("{:?}/25", i);
            v = apply_rules_vec(v);
        }
        Ok(v.len())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // let mut v = parse_input(reader.lines().map(|l| l.unwrap()).next().unwrap().as_str());
        // for i in 0..75 {
        //     println!("{:?}/75", i);
        //     v = apply_rules_vec(v);
        // }
        // Ok(v.len())
        Ok(1)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
