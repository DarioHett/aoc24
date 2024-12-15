use anyhow::*;
use aoc24::day11::*;
use aoc24::*;
use lru::LruCache;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::NonZeroUsize;

const DAY: &str = "11";
const INPUT_FILE: &str = "input/11.txt";

const TEST: &str = "125 17";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let v = parse_input(reader.lines().map(|l| l.unwrap()).next().unwrap().as_str());
        let mut cache = LruCache::new(NonZeroUsize::new(10_000_000).unwrap());
        Ok(v.into_iter()
            .enumerate()
            .map(|(_a, i)| {
                // println!("{:?}", a);
                apply_rules_recur(i, 25, &mut cache)
            })
            .sum::<u64>() as usize)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let v = parse_input(reader.lines().map(|l| l.unwrap()).next().unwrap().as_str());

        let mut cache = LruCache::new(NonZeroUsize::new(10_000_000).unwrap());

        Ok(v.into_iter()
            .enumerate()
            .map(|(_a, i)| {
                // println!("{:?}", a);
                apply_rules_recur(i, 75, &mut cache)
            })
            .sum::<u64>() as usize)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
