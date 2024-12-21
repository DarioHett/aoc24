use anyhow::*;
use aoc24::util::grid::Grid;
use aoc24::util::point::{Point, DOWN, LEFT, RIGHT, UP};
use aoc24::*;
use itertools::Itertools;
use std::fs;

const DAY: &str = "21";
const INPUT_FILE: &str = "input/21.txt";

const TEST: &str = "029A
980A
179A
456A
379A";
// Notes:
// (v-1 % 3, v-1 // 3) = (x, y) for the upper part of grid, e.g. 2 => (1, 0)
// Required shortest paths can always be described in (x2-x1, y2-y1)
// 7 to 2 => (-1, -2)
// `0` is path to `2` + (0, -1), unless starting in `A` or `0`, and vice versa.
// `A` is path to `3` + (0, -1), unless starting in `A` or `0`, and vice versa.
// Use 1 to 9 as regular grid, treat `A` and `0` as hardcoded special cases.

// Keypad moves can be fully hardcoded, opposite direction is simple reversal.
// Could use recursion to ease some of the hardcoding, but grid is small enough for direct style.
// A^, A>, A<*, Av
// ^>, ^<*, ^v
// ><, >v
// <v
// Have to define reversal, v <=> ^; < <=> >

// Directional keypad moves can be applied to themselves to do the expansion
// `0` -> <A
// < A -> <v<A >^>A <<= This is an issue, v<<A is faster in the following expansion
// // Staying on the same key does incur less additional pushes.
// <v<A -> (<v<A >A <A >^>A) 12 moves
// v<<A -> (v<A <A A >^>A) 10 moves
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        Ok(1)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(126384, part1(TEST)?);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2(input: &str) -> Result<usize> {
    //     Ok(1)
    // }
    // assert_eq!(0, part2(TEST)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
