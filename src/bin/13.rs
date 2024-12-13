use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = "input/13.txt";

const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

fn linear_eqn((x1, x2): (i64, i64), (y1, y2): (i64, i64), (z1, z2): (i64, i64)) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    // This should work as any fractional result will be truncated to mismatch.
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<i64> {
        let mut input = &mut "".to_string();
        let _ = reader.read_to_string(input);
        let mut res = 0;
        for l in input.split("\n\n") {
            // May replace w/ regex
            let (x1, x2, y1, y2, z1, z2) = l
                .split(|c: char| !c.is_ascii_digit())
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .unwrap();
            res += linear_eqn((x1, x2), (y1, y2), (z1, z2));
        }
        Ok(res)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(mut reader: R) -> Result<i64> {
    //     Ok(1)
    // }
    //
    // // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = part2(input_file)?;
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
