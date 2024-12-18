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

const TEST2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<String> {
        Ok(sol1(input))
    }

    assert_eq!(String::from("4,6,3,5,6,3,5,2,1,0"), part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2(input: &str) -> Result<u64> {
        let lines = input.lines().collect::<Vec<_>>();

        let a = lines[0][12..].parse::<u64>().unwrap();
        let b = lines[1][12..].parse::<u64>().unwrap();
        let c = lines[2][12..].parse::<u64>().unwrap();

        let program = lines[4][9..]
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut factors = vec![0; program.len()];

        loop {
            let mut init_a = 0;
            for (i, f) in factors.iter().enumerate() {
                init_a += 8u64.pow(i as u32) * f
            }

            let output = run_pt2(init_a, b, c, &program);

            if output == program {
                return Ok(init_a);
            }

            for i in (0..program.len()).rev() {
                if output.len() < i {
                    factors[i] += 1;
                    break;
                }
                if output[i] != program[i] {
                    factors[i] += 1;
                    break;
                }
            }
        }
    }
    //
    assert_eq!(117440, part2(TEST2)?);
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
