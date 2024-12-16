use anyhow::*;
use aoc24::day16::*;
use aoc24::*;
use std::fs;

const DAY: &str = "16";
const INPUT_FILE: &str = "input/16.txt";

const TEST: &str = "";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<i32> {
        Ok(1)
    }

    assert_eq!(0, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2(input: &str) -> Result<i32> {
    //     Ok(1)
    // }
    //
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
