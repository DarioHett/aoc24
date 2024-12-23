use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const DAY: &str = "23";
const INPUT_FILE: &str = "input/23.txt";

const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

pub fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<[u8; 26 * 26]>) {
    let mut adj_list = HashMap::new();
    let mut adj_mat = vec![[0; 26 * 26]; 26 * 26];

    let two_letter_loc = |b: &[u8]| 26 * ((b[0] - b'a') as usize) + ((b[1] - b'a') as usize);

    for line in input.lines() {
        let (source, sink) = line.split_once('-').unwrap();
        let source_ix = two_letter_loc(&source.as_bytes()).to_owned();
        let sink_ix = two_letter_loc(&sink.as_bytes()).to_owned();

        adj_list
            .entry(source_ix)
            .or_insert_with(Vec::new)
            .push(sink_ix);
        adj_list
            .entry(sink_ix)
            .or_insert_with(Vec::new)
            .push(source_ix);

        adj_mat[source_ix][sink_ix] += 1;
        adj_mat[sink_ix][source_ix] += 1;
    }

    (adj_list, adj_mat)
}

pub fn solve_part1(adj_list: &HashMap<usize, Vec<usize>>, adj_mat: &Vec<[u8; 676]>) -> usize {
    let mut counts = [0; 676];
    let mut tri_paths = 0;

    for ix_w_tX in 494..520 {
        if let Some(adjacents) = adj_list.get(&ix_w_tX) {
            counts[ix_w_tX] += 1;

            for (i, &first_step) in adjacents.iter().enumerate() {
                for &second_step in adjacents.iter().skip(i) {
                    if (counts[first_step] == 0)
                        && (counts[second_step] == 0)
                        && (adj_mat[first_step][second_step] > 0)
                    {
                        tri_paths += 1;
                    }
                }
            }
        }
    }

    tri_paths
}

pub fn solve_part2(adj_list: &HashMap<usize, Vec<usize>>, adj_mat: &Vec<[u8; 676]>) -> String {
    let mut counts = [0; 676];
    let mut connected_comp = Vec::new();
    let mut biggest_component = Vec::new();

    for (&first_step, adjacents) in adj_list {
        if (counts[first_step] == 0) {
            connected_comp.clear();
            connected_comp.push(first_step);

            for &second_step in adjacents {
                if connected_comp.iter().all(|&c| adj_mat[second_step][c] > 0) {
                    counts[second_step] += 1;
                    connected_comp.push(second_step);
                }
            }

            if connected_comp.len() > biggest_component.len() {
                biggest_component.clone_from(&connected_comp);
            }
        }
    }

    let mut res = String::new();
    biggest_component.sort_unstable();

    for n in biggest_component {
        res.push(((n / 26) as u8 + b'a') as char);
        res.push(((n % 26) as u8 + b'a') as char);
    }

    res.as_bytes()
        .chunks(2)
        .map(|cs| {
            let mut s = String::new();
            for c in cs {
                s.push(*c as char);
            }
            s
        })
        .collect::<Vec<String>>()
        .join(",")
        .to_string()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let (adj_list, adj_mat) = parse(input);
        Ok(solve_part1(&adj_list, &adj_mat))
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(7, part1(TEST)?);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    println!("\n=== Part 2 ===");

    //region Part 2
    fn part2(input: &str) -> Result<String> {
        let (adj_list, adj_mat) = parse(input);
        Ok(solve_part2(&adj_list, &adj_mat))
    } // assert_eq!(0, part2(TEST)?);
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
