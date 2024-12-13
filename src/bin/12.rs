use anyhow::*;
use aoc24::day12::{
    component_walls, components, components_with_sizes, grid_to_map, horizontal_sides, parse_line,
    size_map, vertical_sides, walk, Walls,
};
use aoc24::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;

const DAY: &str = "12";
const INPUT_FILE: &str = "input/12.txt";

const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = reader
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l.unwrap().as_str(), i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let size_map = size_map(&map);
        let cs = components_with_sizes(&map, &size_map);
        let value = cs.iter().sum();
        Ok(value)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = part1(input_file)?;
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");


    // A region of R plants with price 12 * 10 = 120. x
    // A region of I plants with price 4 * 4 = 16. x
    // A region of C plants with price 14 * 22 = 308. (280), ctr 20
    // A region of F plants with price 10 * 12 = 120. (110), ctr 11
    // A region of V plants with price 13 * 10 = 130. (117), ctr 9
    // A region of J plants with price 11 * 12 = 132. x
    // A region of C plants with price 1 * 4 = 4. x
    // A region of E plants with price 13 * 8 = 104. x
    // A region of I plants with price 14 * 16 = 224. 210, ctr = 15
    // A region of M plants with price 5 * 6 = 30. x
    // A region of S plants with price 3 * 6 = 18. x

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = reader
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l.unwrap().as_str(), i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cs = components(&map);
        let mut res = 0;
        for c in cs.iter() {
            let mut ctr = 0;
            let cwall_map = component_walls(&c);
            cwall_map.iter().for_each(|(w, ps)| match w {
                Walls::Above => ctr += horizontal_sides(ps),
                Walls::Beneath => ctr += horizontal_sides(ps),
                Walls::Left => ctr += vertical_sides(ps),
                Walls::Right => ctr += vertical_sides(ps),
            });
            println!("cwalls = {:?}", cwall_map);
            println!("ctr = {}", ctr);
            println!("val = {}", ctr.mul(c.len()));

            res += ctr.mul(c.len())
        }
        Ok(res)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
