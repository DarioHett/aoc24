pub fn parse_line(input: &str, line_number: usize) -> Vec<(char, usize, usize)> {
    input
        .chars()
        .enumerate()
        .map(|(x, c)| (c, x, line_number))
        .collect::<Vec<_>>()
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Game {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(true);
    }
}
