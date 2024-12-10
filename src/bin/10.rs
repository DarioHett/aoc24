use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = "input/10.txt";

const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let map = reader
            .lines()
            .map(|l| day10::parse(l.unwrap().as_str()))
            .collect::<Vec<Vec<i32>>>();
        let (max_y, max_x) = (map.len() - 1, map[0].len() - 1);
        let (bx, by) = (max_x as i32, max_y as i32);
        let trailheads: Vec<_> = map
            .iter()
            .enumerate()
            .map(|(y, xs)| {
                xs.iter()
                    .enumerate()
                    .filter(|(x, _v)| day10::is_trailhead((*x as i32, y as i32), &map))
                    .map(|(x, _v)| (x as i32, y as i32))
                    .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .collect();
        let mut hm = HashMap::new();
        for trailhead in trailheads {
            hm.insert(trailhead, day10::recursion(trailhead, &map, (&bx, &by)));
        }

        Ok(hm
            .values()
            .map(|v| v.into_iter().unique().count() as u64)
            .sum::<u64>())
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let map = reader
            .lines()
            .map(|l| day10::parse(l.unwrap().as_str()))
            .collect::<Vec<Vec<i32>>>();
        let (max_y, max_x) = (map.len() - 1, map[0].len() - 1);
        let (bx, by) = (max_x as i32, max_y as i32);
        let trailheads: Vec<_> = map
            .iter()
            .enumerate()
            .map(|(y, xs)| {
                xs.iter()
                    .enumerate()
                    .filter(|(x, _v)| day10::is_trailhead((*x as i32, y as i32), &map))
                    .map(|(x, _v)| (x as i32, y as i32))
                    .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .collect();
        let mut hm = HashMap::new();
        for trailhead in trailheads {
            hm.insert(trailhead, day10::recursion2(trailhead, &map, (&bx, &by)));
        }

        Ok(hm
            .values()
            .map(|v| v.into_iter().count() as u64)
            .sum::<u64>())
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
