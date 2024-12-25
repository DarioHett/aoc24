pub fn is_key(schema: &str) -> bool {
    schema.lines().next().unwrap().chars().all(|c| c == '.')
}
pub fn is_lock(schema: &str) -> bool {
    schema.lines().next().unwrap().chars().all(|c| c == '#')
}

pub fn schema_line_to_values(schema_line: &str) -> Vec<u32> {
    schema_line
        .chars()
        .map(|c| {
            if c == '#' {
                1
            } else if c == '.' {
                0
            } else {
                unreachable!()
            }
        })
        .collect()
}

pub fn schema_to_values(schema: &str) -> Vec<u32> {
    let v = schema
        .lines()
        .map(|l| schema_line_to_values(l))
        .collect::<Vec<_>>();
    let mut vals = v[0].clone();
    for sv in v[1..].iter() {
        for (i,s) in sv.iter().enumerate() {
            vals[i] += s
        }
    }
    vals = vals.iter().map(|v| v-1).collect::<Vec<_>>();
    vals
}

pub fn vecs_overlap(lhs: &Vec<u32>, rhs: &Vec<u32>, c: u32) -> bool {
    lhs.iter().zip(rhs.iter()).all(|(l,r)| l + r <= c)
}
mod test {
    use crate::day25::*;

    const KEY: &str = ".....
.....
.....
#....
#.#..
#.#.#
#####";

    const OVERLAP_KEY: &str = ".....
.....
.....
#....
#.#..
###.#
#####";

    const LOCK: &str = "#####
.####
.####
.####
.#.#.
.#...
.....";

    #[test]
    fn test_is_key() {
        assert!(is_key(KEY));
        assert!(is_key(LOCK));
    }

    #[test]
    fn test_is_lock() {
        assert!(!is_lock(KEY));
        assert!(is_lock(LOCK));
    }

    #[test]
    fn test_schema_line_to_values() {
        assert!(schema_to_values(KEY) == vec![3, 0, 2, 0, 1]);
        assert!(schema_to_values(LOCK) == vec![0, 5, 3, 4, 3])
    }

    #[test]
    fn test_vecs_overlap() {
        let kv = schema_to_values(KEY);
        let okv = schema_to_values(OVERLAP_KEY);
        let lv = schema_to_values(LOCK);
        assert!(vecs_overlap(&kv, &lv, 5));
        assert!(!vecs_overlap(&okv, &lv, 5));
    }
}
