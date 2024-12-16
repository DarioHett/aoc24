use anyhow::*;
use aoc24::day16::*;
use aoc24::util::grid::Grid;
use aoc24::util::point::{DOWN, RIGHT, UP};
use aoc24::*;
use std::fs;

const DAY: &str = "16";
const INPUT_FILE: &str = "input/16.txt";

const TEST: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<i32> {
        let grid = Grid::parse(input);
        let cost_grid = &mut Grid::same_size_with(&grid, [i32::MAX; 22]);
        let start = grid.find(b'S').unwrap();
        let end = grid.find(b'E').unwrap();
        let position = &mut start.clone();
        let direction = &mut RIGHT;
        let permissible_directions = &mut [RIGHT, UP, DOWN];

        let _ = direction.mut_clockwise();
        assert_eq!(*direction, DOWN);
        let _ = direction.mut_counter_clockwise();
        assert_eq!(*direction, RIGHT);

        // let current_cost = &mut (grid.height*grid.width*1000);
        let current_cost = &mut 102460;

        let mut path = Vec::new();
        // let i = search(*position, *direction, 0, path, &grid, 0, current_cost);
        let i = search_w_lookup(*position, *direction, 0, path, &grid, 0, current_cost, cost_grid);
        Ok(*current_cost)
    }

    assert_eq!(7036, part1(TEST)?);
    println!("Test done.");

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2(input: &str) -> Result<i32> {
    //     Ok(1)
    // }
    //
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
