use anyhow::*;
use aoc24::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = "input/07.txt";

const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let vs = reader
            .lines()
            .map(|l| {
                let (t, _, sugg) = day07_problem01(l.unwrap().as_str());
                let v = if day07_can_be_made_true(&t, &sugg) {
                    t
                } else {
                    0 as u64
                };
                v
            })
            .collect::<Vec<u64>>();
        Ok(vs.into_iter().sum())
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let map = day07_generate_operators_p2f();
        let vs = reader
            .lines()
            .map(|l| {
                let v = day07_problem02_w_map(l.unwrap().as_str(), &map);
                v
            })
            .collect::<Vec<u64>>();
        Ok(vs.into_iter().sum())
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
