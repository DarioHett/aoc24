use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut the_map = reader
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let nrow = the_map.len();
        let ncol = the_map[0].len();
        let mut next_loc_exceeds_boundaries = false;
        let mut guard = day06_find_guard(&the_map).unwrap();
        let first_guard = Guard::new(guard.pos.clone(), guard.dpos.clone());
        let mut path: Vec<(i32, i32)> = Vec::new();
        while !next_loc_exceeds_boundaries {
            guard.print();
            let (next_x, next_y) = guard.next_loc();
            if (next_x >= (nrow as i32) || next_x < 0 || next_y >= (ncol as i32) || next_y < 0) {
                the_map[guard.pos.1 as usize][guard.pos.0 as usize] = 'X';
                path.push(guard.pos.clone());
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
                path.push(guard.pos.clone());
                guard = guard.step();
                continue;
            }
        }
        // First pass done, all X's identified incl starting pos
        // Only check locations of X's except for start
        path = path
            .iter()
            .unique()
            .cloned()
            .filter(|i| i != &first_guard.pos)
            .collect();
        let n = path.len();
        println!("path = {:?}", path);
        let mut loop_cnt: Vec<(i32, i32)> = Vec::new();
        for (i, loc) in path.iter().enumerate() {
            println!("loop = {:?}, {:?}/{:?}", loc, i, n);
            // Re-insert guard into map
            the_map[first_guard.pos.1 as usize][first_guard.pos.0 as usize] =
                day06_from_orientation(first_guard.dpos.clone());
            let mut counter: HashMap<(i32, i32), i32> = HashMap::new();
            the_map[loc.1 as usize][loc.0 as usize] = '#';
            let nrow = the_map.len();
            let ncol = the_map[0].len();
            let mut next_loc_exceeds_boundaries = false;
            let mut guard = day06_find_guard(&the_map).unwrap();
            while !next_loc_exceeds_boundaries {
                // guard.print();
                let (next_x, next_y) = guard.next_loc();
                if (next_x >= (nrow as i32) || next_x < 0 || next_y >= (ncol as i32) || next_y < 0)
                {
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
                    if let Some(count) = counter.get(&guard.pos) {
                        if (*count > 5) {
                            loop_cnt.push(*loc);
                            break;
                        };
                        counter.insert(guard.pos, count + 1);
                    } else {
                        counter.insert(guard.pos, 1);
                    };
                    guard = guard.step();
                    continue;
                }
            }
            the_map[loc.1 as usize][loc.0 as usize] = 'X';
        }
        println!("{:?}", &loop_cnt);
        Ok(loop_cnt.iter().count() as i32)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
