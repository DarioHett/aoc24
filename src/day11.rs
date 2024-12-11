use std::ops::Div;

pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn number_is_zero(i: u64) -> bool {
    i == 0
}
fn number_of_digits_are_even(i: u64) -> bool {
    u64::ilog10(i) % 2 == 1
}

fn left_split_stones(i: u64) -> u64 {
    i.div(10u64.pow((u64::ilog10(i) + 1) / 2))
}

fn right_split_stones(i: u64) -> u64 {
    i % (10u64.pow((u64::ilog10(i) + 1) / 2))
}

fn third_rule(i: u64) -> u64 {
    i * 2024
}

fn apply_rules(i: u64) -> Vec<u64> {
    if number_is_zero(i) {
        vec![i + 1]
    } else if number_of_digits_are_even(i) {
        vec![left_split_stones(i), right_split_stones(i)]
    } else {
        vec![third_rule(i)]
    }
}

pub fn apply_rules_vec(v: Vec<u64>) -> Vec<u64> {
    v.into_iter().map(apply_rules).flatten().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "125 17";
        let stones = parse_input(line);
        assert_eq!(stones, [125, 17]);
    }

    #[test]
    fn test_apply_rules() {
        let i = 125;
        let v = apply_rules(i);
        assert_eq!(v, [253000]);
        let w = apply_rules(v[0]);
        assert_eq!(w, [253, 0]);
    }

    #[test]
    fn test_apply_rules2() {
        let i = 17;
        let v = apply_rules(i);
        assert_eq!(v, [1, 7]);
        let w = v.into_iter().map(apply_rules).flatten().collect::<Vec<_>>();
        assert_eq!(w, [2024, 14168]);
    }

    #[test]
    fn test_apply_rules_vec() {
        let mut v = vec![125, 17];
        for _ in 0..6 {
            v = apply_rules_vec(v);
        }
        assert_eq!(22, v.len());
        for i in parse_input("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")
        {
            assert!(v.contains(&i));
        }
    }

    #[test]
    fn test_number_of_digits_are_even() {
        assert_eq!(number_of_digits_are_even(1234), true);
        assert_eq!(number_of_digits_are_even(123), false);
        assert_eq!(number_of_digits_are_even(12), true);
        assert_eq!(number_of_digits_are_even(1), false);
    }

    #[test]
    fn test_left_split_stones() {
        assert_eq!(left_split_stones(12), 1);
        assert_eq!(left_split_stones(1234), 12);
        assert_eq!(left_split_stones(123456), 123);
        assert_eq!(left_split_stones(12345678), 1234);
    }

    #[test]
    fn test_right_split_stones() {
        assert_eq!(right_split_stones(12), 2);
        assert_eq!(right_split_stones(1234), 34);
        assert_eq!(right_split_stones(123456), 456);
        assert_eq!(right_split_stones(12345678), 5678);
    }
}
