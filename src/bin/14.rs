use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = "input/14.txt";

const TEST: &str = "";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<i64> {
        Ok(1)
    }

    assert_eq!(0, part1(TEST));;

    let input_file = fs::read_to_string(INPUT_FILE).unwrap();

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(mut reader: R) -> Result<i64> {
    //     Ok(1)
    // }
    //
    // assert_eq!(0, part2(TEST));;
    // let result = part2(input_file.as_str())?;
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
