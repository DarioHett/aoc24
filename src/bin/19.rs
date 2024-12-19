use anyhow::*;
use aoc24::day19::*;
use aoc24::util::grid::Grid;
use aoc24::util::point::Point;
use aoc24::*;
use std::fs;

const DAY: &str = "19";
const INPUT_FILE: &str = "input/19.txt";

const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u32> {
        Ok(1)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(6, part1(TEST)?);

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    // //
    // fn part2(input: &str) -> Result<u64> {
    //     Ok(1)
    // }
    //
    // //
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
