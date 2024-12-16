use crate::util::grid::Grid;
use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};
use itertools::Itertools;
use std::mem::swap;

pub fn rotate_clockwise(direction: &mut Point, dirs: &mut Vec<Point>) {
    direction.mut_clockwise();
    for d in dirs {
        let _ = d.mut_clockwise();
    }
}
pub fn rotate_counter_clockwise(direction: &mut Point, dirs: &mut Vec<Point>) {
    direction.mut_counter_clockwise();
    for d in dirs {
        let _ = d.mut_counter_clockwise();
    }
}

pub fn search(position: Point, direction: Point, cost: i32, mut path: Vec<Point>, map: &Grid<u8>, rotations: u8, current_cost: &mut i32) -> i32 {
    if cost > *current_cost {
        return *current_cost;
    }
    if (map[position] == b'E') {
        println!("FOUND E, COST: {:?}.", cost);
        if cost < *current_cost {
            *current_cost = cost;
        }
        return cost
    };
    if (path.contains(&(position)) || path.contains(&(position+direction)) || map[position] == b'#') {
        return i32::MAX
    };
    println!("Position: {:?}", position);
    let mut new_path = path.clone();
    path.push(position);
    if rotations < 4 {
        search(position+direction, direction, cost+1, path, map, 0, current_cost
        ).min(
            search(position, direction.clockwise(), cost+1000, new_path.clone(), map, rotations+1, current_cost)
        ).min(
            search(position, direction.clockwise().clockwise(), cost+2000, new_path.clone(), map, rotations+2, current_cost)
        ).min(
            search(position, direction.counter_clockwise(), cost+1000, new_path.clone(), map, rotations+1, current_cost)
        )
    }
    else {
        i32::MAX
    }
}

pub fn search_w_lookup(position: Point, direction: Point, cost: i32, mut path: Vec<Point>, map: &Grid<u8>, rotations: u8, current_cost: &mut i32, cost_grid: &mut Grid<[i32; 22]>) -> i32 {
    if cost_grid[position][((direction.x+1)+(direction.y+1)*10) as usize] < cost {
        return i32::MAX
    } else {
        cost_grid[position][((direction.x+1)+(direction.y+1)*10) as usize] = cost;
    }
    if cost > *current_cost {
        return *current_cost;
    }
    if (map[position] == b'E') {
        println!("FOUND E, COST: {:?}.", cost);
        if cost < *current_cost {
            *current_cost = cost;
        }
        return cost
    };
    if (path.contains(&(position)) || path.contains(&(position+direction)) || map[position] == b'#') {
        return i32::MAX
    };
    // println!("Position: {:?}", position);
    let mut new_path = path.clone();
    path.push(position);
    if rotations < 3 {
        crate::day16::search_w_lookup(position+direction, direction, cost+1, path, map, 0, current_cost, cost_grid
        ).min(
            crate::day16::search_w_lookup(position, direction.clockwise(), cost+1000, new_path.clone(), map, rotations+1, current_cost, cost_grid)
        ).min(
            crate::day16::search_w_lookup(position, direction.clockwise().clockwise(), cost+2000, new_path.clone(), map, rotations+2, current_cost, cost_grid)
        ).min(
            crate::day16::search_w_lookup(position, direction.counter_clockwise(), cost+1000, new_path.clone(), map, rotations+1, current_cost, cost_grid)
        )
    }
    else {
        i32::MAX
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::min;
    use std::fs::Permissions;
    use super::*;
    use crate::util::grid::Grid;
    use crate::util::point;
    use crate::util::point::{Point, DOWN, LEFT, RIGHT, UP};

    #[test]
    fn test() {
        let input = "###############
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
        let grid = Grid::parse(input);
        let grid2 = &mut Grid::same_size_with(&grid, [i32::MAX; 22]);
        let start = grid.find(b'S').unwrap();
        let end = grid.find(b'E').unwrap();
        let position = &mut start.clone();
        let direction = &mut RIGHT;
        let permissible_directions = &mut [RIGHT, UP, DOWN];


        let _ = direction.mut_clockwise();
        assert_eq!(*direction, DOWN);
        let _ = direction.mut_counter_clockwise();
        assert_eq!(*direction, RIGHT);

        let current_cost = &mut (i32::MAX-1);


        let mut path = Vec::new();
        let i  = search(*position, *direction, 0, path, &grid, 0, current_cost);
        println!("{:?}",current_cost);
        assert_eq!(7036, *current_cost);


    }
        // for d in permissible_directions.iter() {
        //     if grid[*position + *d] == b'.' {
        //         explorable.push(*d);
        //     }
        // }
        //
        //
        // while explorable.len() > 0 {
        //     let next_direction = explorable.pop().unwrap();
        //     if *direction == next_direction {
        //         current_cost += 1
        //     }

        }




