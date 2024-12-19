use anyhow::*;
use aoc24::day11::parse_input;
use aoc24::day19::*;
use aoc24::*;
use std::fs;

const DAY: &str = "19";
const INPUT_FILE: &str = "input/19.txt";

// Forward pass; abandoned due to complexity (TEST sufficiently small, real data slightly too large)
fn eval_patterns(cur_soln: &Vec<u8>, design: &Vec<u8>, patterns: &Vec<Vec<u8>>) -> u8 {
    let start_char = design[cur_soln.len()];
    // let mut ret_vals = Vec::new();
    for p in patterns
        .iter()
        .filter(|&p| (p.len() + cur_soln.len()) <= design.len())
        .filter(|&p| p[0] == start_char)
        .filter(|&p| design[cur_soln.len()..(cur_soln.len()+p.len())].eq(p))
    {
        if (cur_soln.len() + p.len() == design.len()) {
            println!("1- {:?}", design);
            return 1
        } else {
            let mut new_soln = cur_soln.clone();
            for &c in p {
                new_soln.push(c);
            }
            if eval_patterns(&new_soln, design, patterns) == 1 {
                return 1
            }
        }
    }
    return 0
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let ctr = parse_pt1(input);
        Ok(ctr)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(6, part1(TEST)?);

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2(input: &str) -> Result<usize> {
        Ok(parse_pt2(input))
    }

    //
    assert_eq!(16, part2(TEST)?);
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
