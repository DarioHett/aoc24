use anyhow::*;
use aoc24::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const DAY: &str = "24";
const INPUT_FILE: &str = "input/24.txt";

const TEST: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

fn apply_op(lhs: u8, rhs: u8, op: &str) -> u8 {
    match op {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => unreachable!(),
    }
}
// const LHS_OP_RHS_DEST_REGEX: Regex = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").expect("REASON");

fn parse(input: &str) -> (&str, Vec<[&str; 4]>) {
    let LHS_OP_RHS_DEST_REGEX: Regex = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").expect("REASON");

    let (gate_bits, gate_lhs_op_rhs_dest) = input.split_once("\n\n").unwrap();
    let extract_quad = |line| {
        let (_, quad) = LHS_OP_RHS_DEST_REGEX.captures(line).unwrap().extract::<4>();
        quad
    };
    let gate_quadruples = gate_lhs_op_rhs_dest
        .lines()
        .map(extract_quad)
        .collect::<Vec<_>>();
    (gate_bits, gate_quadruples)
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u64> {
        let mut gate_bit_map: HashMap<&str, u8> = HashMap::new();
        let (gate_bits, mut gate_quadruples) = parse(input);

        gate_bits.lines().for_each(|line: &str| {
            let (name, value) = line.split_once(": ").unwrap();
            gate_bit_map.insert(name, value.parse::<u8>().unwrap());
        });

        // let filter_quads =
        while !gate_quadruples.is_empty() {
            gate_quadruples.retain(|&[lhs, op, rhs, recv]: &[&str; 4]| {
                let left = gate_bit_map.get(lhs);
                let right = gate_bit_map.get(rhs);
                match (left, right) {
                    (Some(&u), Some(&v)) => {
                        gate_bit_map.insert(recv, apply_op(u, v, op));
                        false
                    }
                    _ => true,
                }
            });
        }

        let mut res = 0u64;
        for i in 0i32..64i32 {
            let s = format!("z{i:02}");
            if let Some(&val) = gate_bit_map.get(&s.as_str()) {
                res |= (val as u64) << i
            } else {
                break;
            }
        }

        Ok(res)
    }

    let input_file = fs::read_to_string(INPUT_FILE)?;
    assert_eq!(2024, part1(TEST)?);
    let result = part1(input_file.as_str())?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2(input: &str) -> Result<String> {
        let (_, gate_quadruples) = parse(input);
        let mut gate_bit_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();

        for &[lhs, op, rhs, recv] in gate_quadruples.iter() {
            gate_bit_map.entry(lhs).or_insert(vec![]).push((op, recv));
            gate_bit_map.entry(rhs).or_insert(vec![]).push((op, recv));
        }

        let mut res = vec![];
        for &[lhs, op, rhs, recv] in gate_quadruples.iter() {
            let op_pipe = gate_bit_map.get(&recv);
            let op_in_pipe =
                |op| op_pipe.is_some_and(|v| v.iter().find(|a| a.0 == op).is_some());

            // Conditions
            let has_xor = op_in_pipe("XOR");
            let has_and = op_in_pipe("AND");
            let has_or = op_in_pipe("OR");
            let fst = lhs.ends_with("00") && rhs.ends_with("00");
            let xy_or_yx = (lhs.starts_with('x') && rhs.starts_with('y'))
                || (rhs.starts_with('x') && lhs.starts_with('y'));
            let lst_bits = recv.starts_with('z');
            let lst = recv == "z45";

            let valid = match op {
                "XOR" => {
                    if !xy_or_yx && lst_bits {
                        true
                    } else if xy_or_yx && has_xor {
                        true
                    } else if fst && lst_bits {
                        true
                    } else {
                        false
                    }
                }
                "OR" => {
                    if lst || (has_and && has_xor) {
                        true
                    } else {
                        false
                    }
                }
                "AND" => {
                    if has_or {
                        true
                    } else if fst {
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    unreachable!()
                }
            };
            if !valid {
                res.push(recv);
            }
        }
        res.sort_unstable();
        Ok(res.join(",").to_string())
    }
    let res = part2(input_file.as_str())?;
    println!("Result = {}", res);
    //endregion

    Ok(())
}
