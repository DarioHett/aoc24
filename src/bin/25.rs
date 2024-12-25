use anyhow::*;
use aoc24::*;
use aoc24::day25::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const DAY: &str = "25";
const INPUT_FILE: &str = "input/25.txt";

const TEST: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u64> {
        let mut keys = Vec::with_capacity(1000);
        let mut locks = Vec::with_capacity(1000);
        input.split("\n\n").for_each(|l| {
            if is_lock(l) {
                locks.push(schema_to_values(l));
            } else if is_key(l) {
                keys.push(schema_to_values(l));
            }
            else {
                unreachable!();
            }
        });
        let mut ctr = 0;
        for l in &locks {
            for k in &keys {
              if vecs_overlap(l,k,5) {
                  ctr += 1;
              }
            }
        }
        Ok(ctr)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(3, part1(TEST)?);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2(input: &str) -> Result<u64> {
    //     Ok(1)
    // }
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
