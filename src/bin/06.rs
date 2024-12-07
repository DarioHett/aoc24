use anyhow::*;
use aoc24::*;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

const DAY: &str = "06";
const INPUT_FILE: &str = "input/06.txt";

const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut the_map = reader
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let nrow = the_map.len();
        let ncol = the_map[0].len();
        let mut next_loc_exceeds_boundaries = false;
        let mut guard = day06_find_guard(&the_map).unwrap();
        while !next_loc_exceeds_boundaries {
            guard.print();
            let (next_x, next_y) = guard.next_loc();
            if (next_x >= (nrow as i32) || next_x < 0 || next_y >= (ncol as i32) || next_y < 0) {
                the_map[guard.pos.1 as usize][guard.pos.0 as usize] = 'X';
                next_loc_exceeds_boundaries = true;
                break;
            }

            let c = the_map[next_y as usize][next_x as usize];
            if c == '#' {
                guard = guard.rotate();
                continue;
            }

            if c == '.' || c == 'X' {
                the_map[guard.pos.1 as usize][guard.pos.0 as usize] = 'X';
                guard = guard.step();
                continue;
            }
        }
        Ok(the_map
            .into_iter()
            .map(|r| r.iter().cloned().filter(|i| *i == 'X').count() as i32)
            .sum::<i32>())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    // //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<i32> {
    //     Ok(r)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = part2(input_file)?;
    // println!("Result = {}", result);
    // //endregion

    Ok(())
}
