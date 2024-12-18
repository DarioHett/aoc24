use crate::util::grid::Grid;
use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};
use itertools::Itertools;

pub const BASELINE_COST: u32 = u32::MAX - 1;
pub const UNREACHABLE_COST: u32 = u32::MAX;
pub const CORRUPTED: u8 = b'#';

pub fn input_to_grid(input: &str, n: usize) -> Grid<u8> {
    let coords: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut pts: Vec<Point> = Vec::new();
    for (x, y) in coords {
        pts.push(Point::new(x as i32, y as i32))
    }
    let grid = &mut Grid::new(70 + 1, 70 + 1, b'.');
    for (i, pt) in pts.iter().enumerate() {
        if i < n {
            grid[*pt] = b'#';
        }
    }
    grid.clone()
}

pub fn forward_step(pos: Point, cost: u32, cost_grid: &mut Grid<u32>, grid: &mut Grid<u8>) {
    if cost_grid[pos] == UNREACHABLE_COST {
        return;
    }
    if cost < cost_grid[pos] {
        cost_grid[pos] = cost;
        for d in [UP, DOWN, LEFT, RIGHT] {
            if grid.contains(pos + d) {
                forward_step(pos + d, cost + 1, cost_grid, grid)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::grid::Grid;
    use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let input = "5,4
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
        let grid = &mut input_to_grid(input, 12);
        let cost_grid = &mut Grid::same_size_with(grid, BASELINE_COST);
        for pt in grid.find_all(CORRUPTED) {
            cost_grid[pt] = UNREACHABLE_COST;
        }

        for ln in Grid::to_str(grid).lines() {
            println!("{:?}", ln);
        }
        forward_step(Point::new(0, 0), 0, cost_grid, grid);
        for ln in (&cost_grid).bytes.iter() {
            println!("{:?}", ln);
        }

        let test_value = 22;
    }
}
