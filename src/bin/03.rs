use anyhow::*;
use aoc24::*;
use regex::Regex;
use std::clone::Clone;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "03"; //
const INPUT_FILE: &str = "input/03.txt";

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let res = reader.lines().map(|l| l.unwrap()).collect::<String>();
        let mut vec: Vec<i32> = Vec::new();
        for (_, [l, r]) in re.captures_iter(&res).map(|c| c.extract()) {
            vec.push(i32::from_str(l)? * i32::from_str(r)?)
        }
        Ok(vec.clone().iter().sum())
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))")?;
        let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let res = reader.lines().map(|l| l.unwrap()).collect::<String>();
        let mut vec: Vec<i32> = Vec::new();
        let mut state_do = true;
        for (mtch, [grp]) in re.captures_iter(&res).map(|c| c.extract::<1>()) {
            if (mtch == "do()") {
                state_do = true;
            } else if (mtch == "don't()") {
                state_do = false;
            } else {
                if state_do {
                    println!("{}", mtch);
                    for (_, [l, r]) in re2.captures_iter(&mtch).map(|c| c.extract()) {
                        vec.push(i32::from_str(l)? * i32::from_str(r)?)
                    }
                }
            }
        }
        Ok(vec.clone().iter().sum())
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
