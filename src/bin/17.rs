use anyhow::*;
use aoc24::day17::*;
use aoc24::*;
use std::fs;

const DAY: &str = "17";
const INPUT_FILE: &str = "input/17.txt";

const TEST: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<String> {
        Ok("0")
    }

    assert_eq!(String::from("4,6,3,5,6,3,5,2,1,0"), part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    // fn part2(input: &str) -> Result<String> {
    //     Ok(String::from("Ok"))
    // }
    //
    // assert_eq!(String::from("NotOk"), part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
