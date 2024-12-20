use anyhow::*;
use aoc24::util::grid::Grid;
use aoc24::util::point::{Point, DOWN, LEFT, RIGHT, UP};
use aoc24::*;
use itertools::Itertools;
use std::fs;
use std::path::absolute;

const DAY: &str = "20";
const INPUT_FILE: &str = "input/20.txt";

const TEST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let move_grid = Grid::parse(input);
        let time_grid = &mut move_grid.same_size_with(usize::MAX);

        let mut pos = move_grid.find(b'S').unwrap();
        let fin = move_grid.find(b'E').unwrap();

        // Initial parameters
        let mut cum_time = 0;
        let mut dir = if move_grid[pos + UP] != b'#' {
            UP
        } else if move_grid[pos + DOWN] != b'#' {
            DOWN
        } else if move_grid[pos + LEFT] != b'#' {
            LEFT
        } else {
            RIGHT
        };

        // Initialize time_grid
        while pos != fin {
            time_grid[pos] = cum_time;
            pos = pos + dir;
            cum_time += 1;

            for d in [dir.clockwise(), dir, dir.counter_clockwise()] {
                if move_grid[pos + d] != b'#' {
                    dir = d;
                    break;
                }
            }
        }
        time_grid[pos] = cum_time; // Missed this one

        // Find diffs where either horizontally or vertically 2 steps are > 100
        let mut ctr = 0;
        let dirs = (DOWN * 2, RIGHT * 2);
        for (x, y) in (0..move_grid.width).cartesian_product(0..move_grid.height) {
            let pt = Point::new(x, y);
            if (move_grid[pt] == b'#') {
                continue;
            };
            if x < (move_grid.width - 2) {
                if (move_grid[pt + dirs.1] != b'#') {
                    let saving = (time_grid[pt].abs_diff(time_grid[pt + dirs.1]) - 2);
                    // println!("Found: {:?}:{:?}:{:?}", pt, pt+dirs.1, saving);
                    if (time_grid[pt].abs_diff(time_grid[pt + dirs.1]) - 2) >= 100 {
                        ctr += 1;
                    }
                };
            }
            if y < (move_grid.height - 2) {
                if (move_grid[pt + dirs.0] != b'#') {
                    let saving = (time_grid[pt].abs_diff(time_grid[pt + dirs.0]) - 2);
                    // println!("Found: {:?}:{:?}:{:?}", pt, pt+dirs.0, saving);
                    if saving >= 100 {
                        ctr += 1;
                    }
                };
            }
        }

        Ok(ctr)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    // assert_eq!(0, part1(TEST)?);
    let _ = part1(TEST);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    // //region Part 2
    // println!("\n=== Part 2 ===");
    // //
    // fn part2(input: &str) -> Result<usize> {
    //
    //     Ok(1)
    // }
    //
    // //
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    // //endregion

    Ok(())
}
