use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::fs;
use std::ops::Rem;

const DAY: &str = "14";
const INPUT_FILE: &str = "input/14.txt";

const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
// const TEST: &str = "p=2,4 v=2,-3";

pub fn after_one_second(
    x: &isize,
    y: &isize,
    dx: &isize,
    dy: &isize,
    max_x: &isize,
    max_y: &isize,
) -> (isize, isize) {
    let rest_x = (x + dx).rem(max_x);
    let new_x = if rest_x >= 0 { rest_x } else { max_x + rest_x };
    let rest_y = (y + dy).rem(max_y);
    let new_y = if rest_y >= 0 { rest_y } else { max_y + rest_y };
    (new_x, new_y)
}

pub fn determine_quadrant(x: &isize, y: &isize, max_x: &isize, max_y: &isize) -> u8 {
    let mut q: u8 = 0;
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;
    if x == &mid_x {
        q += 0;
    } else if x < &mid_x {
        q += 2;
    } else if x > &mid_x {
        q += 3;
    }
    if y == &mid_y {
        q *= 0;
    } else if y < &mid_y {
        q *= 5;
    } else if y > &mid_y {
        q *= 7;
    }

    match q {
        0 => 0,
        10 => 1,
        15 => 2,
        14 => 3,
        21 => 4,
        _ => unreachable!(),
    }
}

pub fn make_grid(robots: &Vec<(isize, isize)>, max_x: &isize, max_y: &isize) -> String {
    let mut grid = vec![vec!['.'; *max_x as usize]; *max_y as usize];
    for (x, y) in robots {
        grid[*y as usize][*x as usize] = '#'
    }
    grid.iter_mut()
        .map(|row| {
            row.push('\n');
            row.iter().collect::<String>()
        })
        .collect()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let width: &isize = &101;
        let height: &isize = &103;
        let mut robots: Vec<(isize, isize, isize, isize)> = Vec::new();
        for l in input.split("\n") {
            let vs = l
                .replace("p=", "")
                .replace(" v=", ",")
                .split(',')
                .map(|p| p.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            robots.push(vs);
        }
        println!("{:?}", robots);

        let mut seconds = 100;
        while seconds > 0 {
            seconds -= 1;
            for (x, y, dx, dy) in robots.iter_mut() {
                (*x, *y) = after_one_second(x, y, dx, dy, width, height);
            }
        }
        println!("{:?}", robots);

        let map = robots
            .iter()
            .map(|(x, y, _, _)| (determine_quadrant(x, y, width, height), (x, y)))
            .into_group_map();
        println!("{:?}", map);

        let res = map
            .iter()
            .map(|(k, v)| if *k > 0 { v.len() } else { 1 })
            .product::<usize>();
        Ok(res)
    }

    // assert_eq!(12, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;

    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2(input: &str) -> Result<usize> {
        let width: &isize = &101;
        let height: &isize = &103;
        let mut robots: Vec<(isize, isize, isize, isize)> = Vec::new();
        for l in input.split("\n") {
            let vs = l
                .replace("p=", "")
                .replace(" v=", ",")
                .split(',')
                .map(|p| p.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            robots.push(vs);
        }
        // println!("{:?}", robots);

        let mut seconds = 0;
        while seconds < 10_000 {
            if seconds > 1_000 {
                let coords = robots
                    .iter()
                    .cloned()
                    .map(|(x, y, _, _)| (x, y))
                    .collect::<Vec<(isize, isize)>>();
                let grid = make_grid(&coords, width, height);
                if grid.contains("###############") {
                    return Ok(seconds);
                    // for l in grid.split('\n') {
                    //     println!("{:?}", l);
                    // }
                    //     print!("Step: {:?}\n", seconds);
                    //     let _=stdout().flush();
                    //     let mut s = String::new();
                    //     stdin().read_line(&mut s).expect("Did not enter a correct string");
                }
            }
            for (x, y, dx, dy) in robots.iter_mut() {
                (*x, *y) = after_one_second(x, y, dx, dy, width, height);
            }
            seconds += 1;
        }
        println!("{:?}", robots);

        Ok(1)
    }
    //
    // assert_eq!(0, part2(TEST));;
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
