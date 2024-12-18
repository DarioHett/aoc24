use anyhow::*;
use aoc24::day18::*;
use aoc24::*;
use std::fs;
use aoc24::util::grid::Grid;
use aoc24::util::point::Point;

const DAY: &str = "18";
const INPUT_FILE: &str = "input/18.txt";

const TEST: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u32> {
        let grid = &mut input_to_grid(input, 1024);
        let cost_grid = &mut Grid::same_size_with(grid, BASELINE_COST);
        for pt in grid.find_all(CORRUPTED) {
            cost_grid[pt] = UNREACHABLE_COST;
        }

        for ln in Grid::to_str(grid).lines() {
            println!("{:?}", ln);
        }
        forward_step(Point::new(0,0), 0, cost_grid, grid);
        Ok(cost_grid[Point::new(cost_grid.width-1, cost_grid.height-1)])
    }

    // assert_eq!(0, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    // //
    // fn part2(input: &str) -> Result<u64> {
    //     Ok(1)
    // }
    // //
    // assert_eq!(0, part2(TEST2)?);
    // let res = part2(input_file.as_str())?;
    // println!("Result = {}", res);
    //endregion

    Ok(())
}
