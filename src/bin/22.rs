use anyhow::*;
use aoc24::*;
use std::fs;

const DAY: &str = "22";
const INPUT_FILE: &str = "input/22.txt";

const TEST: &str = "1
10
100
2024";

fn mix_and_prune(mut secret: usize) -> usize {
    secret = (secret ^ (secret << 6)) & 0xffffff;
    secret = (secret ^ (secret >> 5)) & 0xffffff;
    (secret ^ (secret << 11)) & 0xffffff
}

fn loc(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let nos = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut res = 0;
        let mut lookup = vec![u16::MAX; 130321];

        for (id, number) in nos.iter().enumerate() {
            let id = id as u16;

            let zeroth = *number;
            let first = mix_and_prune(zeroth);
            let second = mix_and_prune(first);
            let third = mix_and_prune(second);

            let mut a;
            let mut b = loc(zeroth, first);
            let mut c = loc(first, second);
            let mut d = loc(second, third);

            let mut no = third;
            let mut prev = third % 10;

            for _ in 0..2000-3 {
                no = mix_and_prune(no);
                let px = no % 10;

                (a, b, c, d) = (b, c, d, 9 + px - prev);
                let ix = 6859 * a + 361 * b + 19 * c + d;

                if lookup[ix] != id {
                    lookup[ix] = id;
                }

                prev = px;
            }

            res += no;
        }
        Ok(res)}


    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(37327623, part1(TEST)?);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2(input: &str) -> Result<u16> {
        let nos = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut res = vec![0; 130321];
        let mut lookup = vec![u16::MAX; 130321];

        for (id, no) in nos.iter().enumerate() {
            let id = id as u16;

            let zeroth = *no;
            let mut a;
            let first = mix_and_prune(zeroth);
            let mut b = loc(zeroth, first);
            let second = mix_and_prune(first);
            let mut c = loc(first, second);
            let third = mix_and_prune(second);
            let mut d = loc(second, third);
            let mut no = third;
            let mut prev = third % 10;

            for _ in 0..2000-3 {
                no = mix_and_prune(no);
                let px = no % 10;

                (a, b, c, d) = (b, c, d, 9 + px - prev);
                let ix = 6859 * a + 361 * b + 19 * c + d;

                if lookup[ix] != id {
                    lookup[ix] = id;
                    res[ix] += px as u16;
                }

                prev = px;
            }

        }
        let mut init = vec![0; 130321];
        init.iter_mut().zip(res).for_each(|(a, b)| *a += b);
        Ok(*init.iter().max().unwrap())}
    assert_eq!(24, part2(TEST)?);
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
