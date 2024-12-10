use anyhow::*;
use aoc24::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = "input/05.txt";

const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let (rules, updates) = day05_parse(reader);
        let r = updates
            .iter()
            .filter(|u| day05_is_sorted(u, &rules))
            .map(|u| {
                // let s = day05_sort(u, &rules);
                let n = day05_middle_number(&u);
                println!("{} -> {}", format!("{:?}", u), n);
                n
            })
            .sum::<i32>();
        Ok(r)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let (rules, updates) = day05_parse(reader);
        let r = updates
            .iter()
            .filter(|u| !day05_is_sorted(u, &rules))
            .map(|u| {
                let s = day05_sort(u, &rules);
                let n = day05_middle_number(&s);
                println!("{} -> {}", format!("{:?}", &s), n);
                n
            })
            .sum::<i32>();
        Ok(r)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
