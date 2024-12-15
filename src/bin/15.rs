use anyhow::*;
use aoc24::day15::*;
use aoc24::*;
use std::fs;

const DAY: &str = "15";
const INPUT_FILE: &str = "input/15.txt";

const TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<i32> {
        let (grid, dirs) = grid_and_moves(input);
        let mut new_grid = grid.clone();
        for (_i, direction) in dirs.iter().enumerate() {
            if can_move(find_robot(&new_grid), direction, &new_grid) {
                new_grid = apply_moves(
                    moves(find_robot(&new_grid), direction, &new_grid),
                    direction,
                    &new_grid,
                );
            } else {
                continue;
            }
        }
        let pts = new_grid.find_all(b'O');
        let res = pts.into_iter().map(|p| p.x + p.y * 100).sum();
        Ok(res)
    }

    assert_eq!(10092, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2(input: &str) -> Result<i32> {
        let binding = input
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.");
        let input = binding.as_str();
        let (grid, dirs) = grid_and_moves(input);
        let mut new_grid = grid.clone();
        for (_i, direction) in dirs.iter().enumerate() {

            if can_move_ping(find_robot(&new_grid), direction, &new_grid) {
                let mvs = moves_ping(find_robot(&new_grid), direction, &new_grid);
                new_grid = apply_moves(mvs, direction, &new_grid);
            } else {
                continue;
            }
        }
        println!("Final");
        for l in new_grid.to_str().lines() {
            println!("{}", l);
        }
        let pts = new_grid.find_all(b'[');
        let res = pts.into_iter().map(|p| p.x + p.y * 100).sum();
        Ok(res)
    }

    assert_eq!(9021, part2(TEST)?);
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
