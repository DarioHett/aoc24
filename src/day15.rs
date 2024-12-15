use crate::util::grid::Grid;
use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};
use itertools::Itertools;

pub fn grid_and_moves(input: &str) -> (Grid<u8>, Vec<Point>) {
    let mut grid_str: String = String::new();
    let mut moves: String = String::new();
    let mut parsing_grid = true;

    for l in input.lines() {
        if l == "" {
            parsing_grid = false;
            continue;
        }
        if parsing_grid {
            grid_str.push_str(l);
            grid_str.push_str("\n");
        } else {
            moves.push_str(l);
        }
    }
    let pts: Vec<_> = moves
        .replace("\n", "")
        .bytes()
        .map(|c| Point::from(c))
        .collect();
    let grid = Grid::parse(grid_str.as_str());

    (grid, pts)
}

pub fn find_robot(grid: &Grid<u8>) -> Point {
    grid.find(b'@').unwrap()
}

pub fn can_move(loc: Point, direction: &Point, grid: &Grid<u8>) -> bool {
    if grid[loc + *direction] == b'.' {
        true
    } else if grid[loc + *direction] == b'#' {
        false
    } else {
        can_move(loc + *direction, direction, grid)
    }
}

pub fn can_move2(loc: Point, direction: &Point, grid: &Grid<u8>) -> bool {
    if grid[loc + *direction] == b'.' {
        true
    } else if grid[loc + *direction] == b'#' {
        false
    } else if grid[loc + *direction] == b'[' {
        can_move2(loc + *direction, direction, grid)
            && can_move2(loc + *direction + RIGHT, direction, grid)
    } else {
        can_move2(loc + *direction, direction, grid)
            && can_move2(loc + *direction + LEFT, direction, grid)
    }
}

pub fn can_move_ping(loc: Point, direction: &Point, grid: &Grid<u8>) -> bool {
    if grid[loc + *direction] == b'.' {
        true
    } else if grid[loc + *direction] == b'#' {
        false
    } else if grid[loc + *direction] == b'[' {
        crate::day15::can_move_ping(loc + *direction, direction, grid)
            && crate::day15::can_move_pong(loc + *direction + RIGHT, direction, grid)
    } else {
        crate::day15::can_move_ping(loc + *direction, direction, grid)
            && crate::day15::can_move_pong(loc + *direction + LEFT, direction, grid)
    }
}

pub fn can_move_pong(loc: Point, direction: &Point, grid: &Grid<u8>) -> bool {
    if grid[loc + *direction] == b'.' {
        true
    } else if grid[loc + *direction] == b'#' {
        false
    } else if grid[loc + *direction] == b'[' && (direction == &UP || direction == &DOWN) {
        can_move_ping(loc + *direction, direction, grid) && can_move_ping(loc + *direction + RIGHT, direction, grid)
    } else if grid[loc + *direction] == b']' && (direction == &UP || direction == &DOWN) {
        can_move_ping(loc + *direction, direction, grid) && can_move_ping(loc + *direction + LEFT, direction, grid)
    } else {
        can_move_ping(loc + *direction, direction, grid)
    }
}

pub fn moves(loc: Point, direction: &Point, grid: &Grid<u8>) -> Vec<Point> {
    // Should use `Option` instead
    let mut accumulator = vec![];
    if grid[loc + *direction] == b'.' {
        vec![loc]
    } else if grid[loc + *direction] == b'#' {
        vec![]
    } else {
        let pts = moves(loc + *direction, direction, grid);
        if pts.len() > 0 {
            for pt in moves(loc + *direction, direction, grid).iter().cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator
    }
}

pub fn moves2(loc: Point, direction: &Point, grid: &Grid<u8>) -> Vec<Point> {
    // Should use `Option` instead
    let mut accumulator = vec![];
    if grid[loc + *direction] == b'.' {
        vec![loc]
    } else if grid[loc + *direction] == b'#' {
        vec![]
    } else if grid[loc + *direction] == b']' && (*direction == UP) || (*direction == DOWN) {
        let left_pts = moves2(loc + *direction + LEFT, direction, grid);
        let pts = moves(loc + *direction, direction, grid);
        if pts.len() > 0 && left_pts.len() > 0 {
            for pt in pts.iter().cloned() {
                accumulator.push(pt);
            }
            for pt in left_pts.iter().cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator
    } else if grid[loc + *direction] == b'[' && (*direction == UP) || (*direction == DOWN) {
        let right_pts = moves2(loc + *direction + RIGHT, direction, grid);
        let pts = moves2(loc + *direction, direction, grid);
        if pts.len() > 0 && right_pts.len() > 0 {
            for pt in pts.iter().cloned() {
                accumulator.push(pt);
            }
            for pt in right_pts.iter().cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator
    } else {
        let pts = moves(loc + *direction, direction, grid);
        if pts.len() > 0 {
            for pt in moves(loc + *direction, direction, grid).iter().cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator
    }
}

pub fn moves_ping(loc: Point, direction: &Point, grid: &Grid<u8>) -> Vec<Point> {
    // Should use `Option` instead
    let mut accumulator = vec![];
    if grid[loc + *direction] == b'.' {
        vec![loc]
    } else if grid[loc + *direction] == b'#' {
        vec![]
    } else if grid[loc + *direction] == b']' && ((*direction == UP) || (*direction == DOWN)) {
        let left_pts = moves_ping(loc + *direction + LEFT, direction, grid);
        let pts = moves_ping(loc + *direction, direction, grid);
        if pts.len() > 0 && left_pts.len() > 0 {
            for pt in pts.iter().interleave(left_pts.iter()).cloned() {
                if !accumulator.contains(&pt) {accumulator.push(pt);}
            }
            if !accumulator.contains(&loc) {accumulator.push(loc);}
            }
            accumulator.dedup();
            accumulator
    } else if grid[loc + *direction] == b'[' && ((*direction == UP) || (*direction == DOWN)) {
        let left_pts = moves_ping(loc + *direction + RIGHT, direction, grid);
        let pts = moves_ping(loc + *direction, direction, grid);
        if pts.len() > 0 && left_pts.len() > 0 {
            for pt in pts.iter().interleave(left_pts.iter()).cloned() {
                if !accumulator.contains(&pt) {accumulator.push(pt);}
            }
            accumulator.push(loc);
        }
        accumulator.dedup();
        accumulator
    } else {
        let pts = moves_ping(loc + *direction, direction, grid);
        if pts.len() > 0 {
            for pt in moves_ping(loc + *direction, direction, grid)
                .iter()
                .cloned()
            {
                if !accumulator.contains(&pt) {accumulator.push(pt);}
            }
            if !accumulator.contains(&loc) {accumulator.push(loc);}
        }
        accumulator.dedup();
        accumulator
    }
}

pub fn moves_pong2(loc: Point, direction: &Point, grid: &Grid<u8>) -> Vec<Point> {
    let mut accumulator = vec![];
    if grid[loc + *direction] == b'.' {
        vec![loc]
    } else if grid[loc + *direction] == b'#' {
        vec![]
    } else if grid[loc + *direction] == b']' && ((*direction == UP) || (*direction == DOWN)) {
        let mut left_pts = Vec::new();
        if grid[loc] != grid[loc + *direction] {
            for pt in moves_ping(loc + *direction + LEFT, direction, grid)
                .iter()
                .cloned()
            {
                left_pts.push(pt);
            }
        };
        let pts = moves_pong2(loc + *direction, direction, grid);
        if pts.len() > 0 && left_pts.len() > 0 {
            for pt in pts.iter().interleave(left_pts.iter()).cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator.dedup();
        accumulator
    } else if grid[loc + *direction] == b'[' && ((*direction == UP) || (*direction == DOWN)) {
        let mut right_pts = Vec::new();
        if grid[loc] != grid[loc + *direction] {
            for pt in moves_ping(loc + *direction + RIGHT, direction, grid) {
                right_pts.push(pt);
            }
        };
        let pts = moves_pong2(loc + *direction, direction, grid);
        if pts.len() > 0 && right_pts.len() > 0 {
            for pt in pts.iter().interleave(right_pts.iter()).cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator.dedup();
        accumulator
    } else {
        let pts = moves_ping(loc + *direction, direction, grid);
        if pts.len() > 0 {
            for pt in moves_ping(loc + *direction, direction, grid)
                .iter()
                .cloned()
            {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator.dedup();
        accumulator
    }
}
pub fn moves_pong(loc: Point, direction: &Point, grid: &Grid<u8>) -> Vec<Point> {
    // Should use `Option` instead
    let mut accumulator = vec![];
    if grid[loc + *direction] == b'.' {
        vec![loc]
    } else if grid[loc + *direction] == b'#' {
        vec![]
    } else {
        let pts = moves_ping(loc + *direction, direction, grid);
        if pts.len() > 0 {
            for pt in pts.iter().cloned() {
                accumulator.push(pt);
            }
            accumulator.push(loc);
        }
        accumulator.dedup();
        accumulator
    }
}

pub fn apply_moves(mvs: Vec<Point>, direction: &Point, grid: &Grid<u8>) -> Grid<u8> {
    let mut new_grid = grid.clone();
    let mut swap_buffer: u8;
    for mv in mvs.iter() {
        swap_buffer = new_grid[*mv + *direction];
        new_grid[*mv + *direction] = grid[*mv];
        new_grid[*mv] = swap_buffer;
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::grid::Grid;
    use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};

    #[test]
    fn test_parse_grid() {
        const TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";
        let grid = Grid::parse(TEST);
        assert_eq!(grid[Point::new(0, 0)], b'#');
        assert_eq!(grid[Point::new(1, 1)], b'.');
        assert_eq!(grid[Point::new(3, 1)], b'O');
    }

    #[test]
    fn test_parse_moves() {
        const TEST: &str = "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let pts: Vec<_> = TEST
            .replace("\n", "")
            .bytes()
            .map(|c| Point::from(c))
            .collect();
        assert_eq!(pts[0], LEFT);
        assert_eq!(pts[1], DOWN);
        assert_eq!(pts[3], RIGHT);
        assert_eq!(pts[pts.len() - 2], LEFT);
        assert_eq!(pts[pts.len() - 1], UP);
    }

    #[test]
    fn test_split_grid_and_moves() {
        const TEST: &str = "##

<>
<<";

        let mut grid: String = String::new();
        let mut moves: String = String::new();
        let mut parsing_grid = true;

        for l in TEST.lines() {
            if l == "" {
                parsing_grid = false;
                continue;
            }
            if parsing_grid {
                grid.push_str(l);
            } else {
                moves.push_str(l);
            }
        }

        assert_eq!(grid, "##");
        assert_eq!(moves, "<><<");
    }
    #[test]
    fn test_fn_grid_and_moves() {
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
        let (grid, pts) = grid_and_moves(TEST);
        assert_eq!(pts[0], LEFT);
        assert_eq!(pts[1], DOWN);
        assert_eq!(pts[3], RIGHT);
        assert_eq!(pts[pts.len() - 2], LEFT);
        assert_eq!(pts[pts.len() - 1], UP);
        assert_eq!(grid[Point::new(0, 0)], b'#');
        assert_eq!(grid[Point::new(1, 1)], b'.');
        assert_eq!(grid[Point::new(3, 1)], b'O');
        assert_eq!(grid.find(b'@').unwrap(), Point { x: 4, y: 4 });
        assert_ne!(grid.find(b'@').unwrap(), Point { x: 3, y: 4 });
    }

    #[test]
    fn test_can_move() {
        let input = "#.O@";
        let grid = Grid::parse(input);
        assert!(can_move(find_robot(&grid), &LEFT, &grid));
        let input = "#OO@";
        let grid = Grid::parse(input);
        assert!(!can_move(find_robot(&grid), &LEFT, &grid))
    }
    #[test]
    fn test_moves() {
        let input = "..O@";
        let grid = Grid::parse(input);
        assert_eq!(
            moves(find_robot(&grid), &LEFT, &grid),
            [Point { x: 2, y: 0 }, Point { x: 3, y: 0 }]
        );
        let input = ".#O@";
        let grid = Grid::parse(input);
        assert_eq!(moves(find_robot(&grid), &LEFT, &grid), []);
        let input = ".OO@";
        let grid = Grid::parse(input);
        assert_eq!(
            moves(find_robot(&grid), &LEFT, &grid),
            [
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 }
            ]
        );
        let input = "O.O@";
        let grid = Grid::parse(input);
        assert_eq!(
            moves(find_robot(&grid), &LEFT, &grid),
            [Point { x: 2, y: 0 }, Point { x: 3, y: 0 }]
        );
        let input = "O.@O";
        let grid = Grid::parse(input);
        assert_eq!(
            moves(find_robot(&grid), &LEFT, &grid),
            [Point { x: 2, y: 0 }]
        );
        let input = "OO.@";
        let grid = Grid::parse(input);
        assert_eq!(
            moves(find_robot(&grid), &LEFT, &grid),
            [Point { x: 3, y: 0 }]
        );
    }

    #[test]
    fn test_apply_moves() {
        let input = "..O@";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), ".O@.");
        let input = ".#O@";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), ".#O@");
        let input = ".OO@";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), "OO@.");
        let input = "O.O@";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), "OO@.");
        let input = "O.@O";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), "O@.O");
        let input = "OO.@";
        let grid = Grid::parse(input);
        let new_grid = apply_moves(moves(find_robot(&grid), &LEFT, &grid), &LEFT, &grid);
        assert_eq!(String::from_utf8(new_grid.bytes).unwrap(), "OO@.");
    }

    #[test]
    fn test_print_grid() {
        let input = "..O@
####";
        let grid = Grid::parse(input);
        let gridstr: Vec<_> = grid
            .bytes
            .chunks_exact(grid.width as usize)
            .map(|row| String::from_utf8(row.to_vec()).unwrap())
            .collect();
        assert!(true)
    }
}
