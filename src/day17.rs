use itertools::Itertools;
use std::ops::Rem;

pub fn get_combo_operand(
    operand: u8,
    register_A: &mut u32,
    register_B: &mut u32,
    register_C: &mut u32,
) -> u32 {
    if (operand < 4) && (operand >= 0) {
        operand as u32
    } else if (operand == 4) {
        *register_A
    } else if (operand == 5) {
        *register_B
    } else if (operand == 6) {
        *register_C
    } else {
        unreachable!();
    }
}

pub fn handle_instruction(
    opcode: u8,
    operand: u8,
    register_A: &mut u32,
    register_B: &mut u32,
    register_C: &mut u32,
    index: &mut usize,
    console: &mut String,
) {
    if opcode == 0 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_A = num / denom;
        *index += 1;
        return;
    }
    if opcode == 1 {
        *register_B = *register_B ^ (operand as u32);
        *index += 1;
        return;
    }
    if opcode == 2 {
        *register_B = get_combo_operand(operand, register_A, register_B, register_C).rem(8);
        *index += 1;
        return;
    }
    if opcode == 3 {
        if *register_A == 0 {
            *index = *index + 1;
            return;
        }
        *index = (operand as usize).div_euclid(2);
        return;
    }
    if opcode == 4 {
        *register_B = *register_B ^ *register_C;
        *index += 1;
        return;
    }
    if opcode == 5 {
        let out = get_combo_operand(operand, register_A, register_B, register_C).rem(8);
        if !console.is_empty() {
            console.push_str(",");
        }
        console.push_str(&out.to_string());
        // println!("{:?}", (*console).as_str());
        *index = *index + 1;
        return;
    }
    if opcode == 6 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_B = num / denom;
        *index += 1;
        return;
    }
    if opcode == 7 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_C = num / denom;
        *index += 1;
        return;
    }
    unreachable!()
}

pub fn handle_instruction2(
    opcode: u8,
    operand: u8,
    register_A: &mut u32,
    register_B: &mut u32,
    register_C: &mut u32,
    index: &mut usize,
    console: &mut Vec<u8>,
) {
    if opcode == 0 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_A = num / denom;
        *index += 1;
        return;
    }
    if opcode == 1 {
        *register_B = *register_B ^ (operand as u32);
        *index += 1;
        return;
    }
    if opcode == 2 {
        *register_B = get_combo_operand(operand, register_A, register_B, register_C).rem(8);
        *index += 1;
        return;
    }
    if opcode == 3 {
        if *register_A == 0 {
            *index = *index + 1;
            return;
        }
        *index = (operand as usize).div_euclid(2);
        return;
    }
    if opcode == 4 {
        *register_B = *register_B ^ *register_C;
        *index += 1;
        return;
    }
    if opcode == 5 {
        let out = get_combo_operand(operand, register_A, register_B, register_C).rem(8) as u8;
        console.push(out);
        *index = *index + 1;
        return;
    }
    if opcode == 6 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_B = num / denom;
        *index += 1;
        return;
    }
    if opcode == 7 {
        let num = *register_A;
        let denom = 2u32.pow(get_combo_operand(
            operand, register_A, register_B, register_C,
        ));
        *register_C = num / denom;
        *index += 1;
        return;
    }
    unreachable!()
}

pub fn sol1(input: &str) -> String {
    let rA: &mut u32;
    let rB: &mut u32;
    let rC: &mut u32;
    let output = &mut String::new();
    let ix = &mut 0usize;
    let mut opcodes: Vec<u8> = Vec::new();
    let mut operands: Vec<u8> = Vec::new();
    let mut ls = input.lines();
    let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
    let mut binding = r.parse::<u32>().unwrap();
    rA = &mut binding;
    let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
    let mut binding = r.parse::<u32>().unwrap();
    rB = &mut binding;
    let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
    let mut binding = r.parse::<u32>().unwrap();
    rC = &mut binding;
    let (_, r) = ls.skip(1).next().unwrap().split_once(": ").unwrap();
    let vs = r.split(",").map(|c| c.parse::<u8>().unwrap()).collect_vec();
    for (i, v) in vs.into_iter().enumerate() {
        if i % 2 == 0 {
            opcodes.push(v);
        } else {
            operands.push(v);
        }
    }

    while *ix < opcodes.len() {
        handle_instruction(opcodes[*ix], operands[*ix], rA, rB, rC, ix, output)
    }
    output.to_string()
}

pub fn run_pt2(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip + 1];

        let combo = match literal {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => literal, // invalid combo - pass through
        };

        match opcode {
            0 => a /= 2u64.pow(combo as u32),
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a / 2u64.pow(combo as u32),
            7 => c = a / 2u64.pow(combo as u32),
            _ => panic!(),
        };

        ip += 2;
    }

    output
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let rA: &mut u32;
        let rB: &mut u32;
        let rC: &mut u32;
        let mut ix = 0usize;
        let mut opcodes: Vec<u8> = Vec::new();
        let mut operands: Vec<u8> = Vec::new();
        let mut ls = input.lines();
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rA = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rB = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rC = &mut binding;
        let (_, r) = ls.skip(1).next().unwrap().split_once(": ").unwrap();
        let vs = r.split(",").map(|c| c.parse::<u8>().unwrap()).collect_vec();
        for (i, v) in vs.into_iter().enumerate() {
            if i % 2 == 0 {
                opcodes.push(v);
            } else {
                operands.push(v);
            }
        }
        assert_eq!(vec![0, 5, 3], opcodes);
        assert_eq!(vec![1, 4, 0], operands);
        assert_eq!(729, *rA);
        assert_eq!(0, *rB);
        assert_eq!(0, *rC);
    }

    #[test]
    fn test2() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let rA: &mut u32;
        let rB: &mut u32;
        let rC: &mut u32;
        let output = &mut String::new();
        let ix = &mut 0usize;
        let mut opcodes: Vec<u8> = Vec::new();
        let mut operands: Vec<u8> = Vec::new();
        let mut ls = input.lines();
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rA = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rB = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rC = &mut binding;
        let (_, r) = ls.skip(1).next().unwrap().split_once(": ").unwrap();
        let vs = r.split(",").map(|c| c.parse::<u8>().unwrap()).collect_vec();
        for (i, v) in vs.into_iter().enumerate() {
            if i % 2 == 0 {
                opcodes.push(v);
            } else {
                operands.push(v);
            }
        }
        assert_eq!(vec![0, 5, 3], opcodes);
        assert_eq!(vec![1, 4, 0], operands);
        assert_eq!(729, *rA);
        assert_eq!(0, *rB);
        assert_eq!(0, *rC);

        while *ix < opcodes.len() {
            handle_instruction(opcodes[*ix], operands[*ix], rA, rB, rC, ix, output)
        }
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test3() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        let rA: &mut u32;
        let rB: &mut u32;
        let rC: &mut u32;
        let output = &mut String::new();
        let ix = &mut 0usize;
        let mut opcodes: Vec<u8> = Vec::new();
        let mut operands: Vec<u8> = Vec::new();
        let mut ls = input.lines();
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rA = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rB = &mut binding;
        let (_, r) = ls.next().unwrap().split_once(": ").unwrap();
        let mut binding = r.parse::<u32>().unwrap();
        rC = &mut binding;
        let (_, r) = ls.skip(1).next().unwrap().split_once(": ").unwrap();
        let vs = r.split(",").map(|c| c.parse::<u8>().unwrap()).collect_vec();
        for (i, v) in vs.into_iter().enumerate() {
            if i % 2 == 0 {
                opcodes.push(v);
            } else {
                operands.push(v);
            }
        }

        while *ix < opcodes.len() {
            handle_instruction(opcodes[*ix], operands[*ix], rA, rB, rC, ix, output)
        }
        assert_eq!(output, "0,1,2");
    }
}
