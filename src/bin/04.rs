use anyhow::*;
use aoc24::*;
use std::clone::Clone;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "04";
const INPUT_FILE: &str = "input/04.txt";

const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let res: Vec<Vec<char>> = reader
            .lines()
            .map(|s| s.unwrap().chars().collect())
            .collect::<Vec<Vec<char>>>();
        let ncols = res.len();
        let nrows = res[0].len();
        let mut no_xmas = 0;
        for col in 0..ncols {
            for row in 0..nrows {
                if res[col][row] == 'X' {
                    if row <= (nrows - 1) - 3 {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col][row + 1] == 'M')
                                && (res[col][row + 2] == 'A')
                                && (res[col][row + 3] == 'S')) as i32
                    }
                    if row >= 3 {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col][row - 1] == 'M')
                                && (res[col][row - 2] == 'A')
                                && (res[col][row - 3] == 'S')) as i32
                    }
                    if col <= (ncols - 1) - 3 {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col + 1][row] == 'M')
                                && (res[col + 2][row] == 'A')
                                && (res[col + 3][row] == 'S')) as i32
                    }
                    if col >= 3 {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col - 1][row] == 'M')
                                && (res[col - 2][row] == 'A')
                                && (res[col - 3][row] == 'S')) as i32
                    }
                    if (col >= 3) & (row >= 3) {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col - 1][row - 1] == 'M')
                                && (res[col - 2][row - 2] == 'A')
                                && (res[col - 3][row - 3] == 'S'))
                                as i32
                    }
                    if (col >= 3) & (row <= nrows - 1 - 3) {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col - 1][row + 1] == 'M')
                                && (res[col - 2][row + 2] == 'A')
                                && (res[col - 3][row + 3] == 'S'))
                                as i32
                    }
                    if (col <= ncols - 1 - 3) & (row <= nrows - 1 - 3) {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col + 1][row + 1] == 'M')
                                && (res[col + 2][row + 2] == 'A')
                                && (res[col + 3][row + 3] == 'S'))
                                as i32
                    }
                    if (col <= ncols - 1 - 3) & (row >= 3) {
                        no_xmas = no_xmas
                            + ((res[col][row] == 'X')
                                && (res[col + 1][row - 1] == 'M')
                                && (res[col + 2][row - 2] == 'A')
                                && (res[col + 3][row - 3] == 'S'))
                                as i32
                    }
                }
            }
        }
        Ok(no_xmas)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion
    //
    // //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<i32> {
    //     let res = reader.lines().count();
    //     Ok(res as i32)
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
