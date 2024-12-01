use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

const DAY: &str = "01"; //
const INPUT_FILE: &str = "input/01.txt";

const TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        let left_list = lines
            .clone()
            .into_iter()
            .map(|l| l.splitn(2, "   ").next().unwrap().parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<i32>>();
        let right_list = lines
            .clone()
            .into_iter()
            .map(|l| l.splitn(2, "   ").nth(1).unwrap().parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<i32>>();
        let result = zip(left_list, right_list)
            .map(|(l, r)| (l - r).abs() as u32)
            .sum();

        Ok(result)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        let left_list = lines
            .clone()
            .into_iter()
            .map(|l| l.splitn(2, "   ").next().unwrap().parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<i32>>();
        let right_list = lines
            .clone()
            .into_iter()
            .map(|l| l.splitn(2, "   ").nth(1).unwrap().parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<i32>>();
        let right_counter: HashMap<i32, i32> =
            right_list.into_iter().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
        let result = left_list
            .iter()
            .map(|v| right_counter.get(v).unwrap_or(&0) * v)
            .sum::<i32>() as u32;
        Ok(result)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
