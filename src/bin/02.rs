use anyhow::*;
use aoc24::*;
use std::clone::Clone;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "02";
const INPUT_FILE: &str = "input/02.txt";

const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
61 58 55 56 54 53
64 58 56 55 54 53
48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    .map(i32::from_str)
                    .fold(Day02State::Initial, |acc, v| day02_is_safe(acc, v.unwrap()))
            })
            .collect::<Vec<Day02State>>();
        Ok(answer.len()
            - answer
                .iter()
                .map(|i| i.eq(&Day02State::Unsafe) as usize)
                .sum::<usize>())
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|line| {
                let l = line.unwrap();
                let k = l
                    .split(' ')
                    .map(i32::from_str)
                    .map(|x| x.unwrap())
                    .collect::<Vec<i32>>();
                let mut x = Day02State::Unsafe;
                for i in 0..l.len() {
                    let v = k
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| !i.eq(j))
                        .map(|(_, v)| v.clone())
                        .fold(Day02State::Initial, |acc, v| day02_is_safe(acc, v));
                    if !v.eq(&Day02State::Unsafe) {
                        x = v.clone()
                    }
                }
                return x;
            })
            .collect::<Vec<Day02State>>();
        let n = answer.len();

        Ok(n - answer
            .into_iter()
            .map(|(l)| l.eq(&Day02State::Unsafe) as usize)
            .sum::<usize>())
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
