use std::char::from_u32;

pub const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

pub fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut ln = input.lines();
    let patterns = ln
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let desings = ln
        .skip(1)
        .map(|s| s.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    (patterns, desings)
}

fn is_pattern_valid(suggested_pattern: &str, loc: usize, design: &str) -> bool {
    if (loc + suggested_pattern.len() > design.len()) {
        return false;
    }
    let pattern_validity = (String::from(suggested_pattern)
        .as_mut()
        .chars()
        .enumerate()
        .fold(true, |acc, (i, c)| {
            acc && (design.chars().nth(loc + i).unwrap() == c)
        }));
    pattern_validity
}

fn add_candidate(
    cand: &Vec<u8>,
    loc: usize,
    current_solution: &Vec<u8>,
    target: &Vec<u8>,
) -> Option<Vec<u8>> {
    if (current_solution.len() + cand.len() <= target.len()) {
        let mut potential_solution = current_solution.clone();
        potential_solution.resize(current_solution.len() + cand.len(), 0);
        for (i, &v) in cand.iter().enumerate() {
            potential_solution[loc + i] = v
        }
        Some(potential_solution)
    } else {
        None
    }
}

fn add_cands(
    cands: &Vec<Vec<u8>>,
    loc: usize,
    current_solution: &Vec<u8>,
    target: &Vec<u8>,
) -> Option<Vec<Vec<u8>>> {
    let mut solns = Vec::new();
    for c in cands {
        if let Some(solution) = add_candidate(c, loc, current_solution, target) {
            solns.push(solution);
        }
    }
    if solns.is_empty() {
        return None;
    }
    Some(solns)
}

pub fn design_pattern_map(design: &Vec<u8>, patterns: &Vec<Vec<u8>>) -> Vec<Vec<Vec<u8>>> {
    let mut tbl = Vec::with_capacity(design.len());
    for (i, &b) in design.iter().enumerate() {
        let mut lookup = Vec::new();
        let remaining_bytes = design.len() - i;
        for p in patterns {
            if !is_pattern_valid(
                p.into_iter()
                    .map(|&e| from_u32(e as u32).unwrap())
                    .collect::<String>()
                    .as_str(),
                i,
                design
                    .into_iter()
                    .map(|&e| from_u32(e as u32).unwrap())
                    .collect::<String>()
                    .as_str(),
            ) {
                continue;
            }
            if p.len() > remaining_bytes {
                continue;
            }
            if p[0] != b {
                continue;
            }
            lookup.push(p.clone());
        }
        tbl.push(lookup);
    }
    tbl
}

pub fn run(candidate_sets: &Vec<Vec<Vec<u8>>>, target: &Vec<u8>) -> Vec<Vec<u8>> {
    let presolns = add_cands(&candidate_sets[0], 0, &Vec::new(), target);
    if presolns.is_none() {
        return vec![];
    }
    let mut solns = presolns.unwrap();
    for i in 1..candidate_sets.len() {
        let mut sol_buffer = Vec::new();
        for s in solns {
            if s.len() == i {
                let additional_solns = add_cands(&candidate_sets[i], i, &s.clone(), target);
                if additional_solns.is_some() {
                    for t in additional_solns.unwrap() {
                        sol_buffer.push(t.clone());
                    }
                }
            } else {
                sol_buffer.push(s)
            }
        }
        solns = sol_buffer;
    }
    solns
}

pub fn parse_pt1(input: &str) -> usize {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut lookup = Vec::new();
    lookup.push(NextCell::new());

    for pattern in patterns.split(", ") {
        let mut i = 0;

        for j in pattern.bytes().map(compress) {
            if lookup[i].next[j] == 0 {
                lookup[i].next[j] = lookup.len();
                i = lookup.len();
                lookup.push(NextCell::new());
            } else {
                i = lookup[i].next[j];
            }
        }

        lookup[i].set_pattern();
    }

    let mut ctr = 0;
    let mut ways = Vec::new();

    for design in designs.lines().map(str::as_bytes) {
        let size = design.len();

        ways.clear();
        ways.resize(size + 1, 0);
        ways[0] = 1;

        for start in 0..size {
            if ways[start] > 0 {
                let mut i = 0;

                for end in start..size {
                    i = lookup[i].next[compress(design[end])];
                    if i == 0 {
                        break;
                    }
                    ways[end + 1] += lookup[i].pattern() * ways[start];
                }
            }
        }

        let total = ways[size];
        ctr += (total > 0) as usize;
    }

    ctr
}

pub fn parse_pt2(input: &str) -> usize {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut lookup = Vec::new();
    lookup.push(NextCell::new());

    for pattern in patterns.split(", ") {
        let mut i = 0;

        for j in pattern.bytes().map(compress) {
            if lookup[i].next[j] == 0 {
                lookup[i].next[j] = lookup.len();
                i = lookup.len();
                lookup.push(NextCell::new());
            } else {
                i = lookup[i].next[j];
            }
        }

        lookup[i].set_pattern();
    }

    let mut ctr = 0;
    let mut ways = Vec::new();

    for design in designs.lines().map(str::as_bytes) {
        let size = design.len();

        ways.clear();
        ways.resize(size + 1, 0);
        ways[0] = 1;

        for start in 0..size {
            if ways[start] > 0 {
                let mut i = 0;

                for end in start..size {
                    i = lookup[i].next[compress(design[end])];
                    if i == 0 {
                        break;
                    }
                    ways[end + 1] += lookup[i].pattern() * ways[start];
                }
            }
        }

        ctr += ways[size];
    }

    ctr
}

fn compress(b: u8) -> usize {
    let n = b as usize;
    (n ^ (n >> 4)) % 8
}

struct NextCell {
    next: [usize; 6],
}

impl NextCell {
    fn new() -> Self {
        NextCell { next: [0; 6] }
    }

    fn set_pattern(&mut self) {
        self.next[3] = 1;
    }

    fn pattern(&self) -> usize {
        self.next[3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
