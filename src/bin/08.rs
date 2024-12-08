use anyhow::*;
use aoc24::char_map_rep::CharMapRep;
use aoc24::*;
use itertools::Itertools;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = "input/08.txt";

const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let charmap = reader
            .lines()
            .into_iter()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let maprep = CharMapRep::new(charmap, '.');
        println!("{:?}", maprep.items());
        let item_location_distance_pair_vect =
            maprep.items().into_iter().map(|i| {
                maprep
                    .item_locs(i)
                    .unwrap()
                    .into_iter().permutations(2                    )
                    .map(|p| (p[0], p[1]))
                    .map(|((xa,ya),(xb,yb))| ((*xa as i32,*ya as i32),(*xb as i32-*xa as i32,*yb as i32 - *ya as i32)))
                    .collect::<Vec<_>>()
            }).flatten().collect::<Vec<_>>();
        println!("{:?}", item_location_distance_pair_vect);

        let mut item_location_distance_pairs: Vec<((i32, i32), (i32, i32))> =
         Vec::new();
        item_location_distance_pair_vect.iter().for_each(
            |((lk,rk),v): &((i32, i32), (i32,i32))| { item_location_distance_pairs.push(((lk.clone(),rk.clone()), v.clone())); });
        println!("{:?}", item_location_distance_pairs);

        let res: Vec<_> = item_location_distance_pairs.iter().map(
            |((x,y),(dx,dy))| ((*x as i32 - dx), (*y as i32 - dy)))
        .filter(|(x,y)| maprep.is_loc((*x as usize, *y as usize))
        ).sorted().dedup().collect();
        println!("{:?}", res);
        Ok(res.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<u64> {
    //     Ok(1)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = part2(input_file)?;
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
