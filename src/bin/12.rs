use anyhow::*;
use aoc24::day12::{
    component_walls, components, components_with_sizes, grid_to_map, horizontal_sides, parse_line,
    size_map, vertical_sides, Walls,
};
use aoc24::*;
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
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

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
