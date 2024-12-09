use itertools::repeat_n;

pub enum DiskMapState {
    DiskDigit(u32),
    Empty(u32),
}

fn expand_disk_map(map: &str) -> String {
    let mut state = DiskMapState::DiskDigit(0);
    let substrs = map
        .chars()
        .map(|c| {
            let n = c.to_digit(10).unwrap() as usize;
            let v = match state {
                DiskMapState::DiskDigit(d) => {
                    state = DiskMapState::Empty(d + 1);
                    repeat_n(d.to_string(), n).collect::<String>()
                }
                DiskMapState::Empty(d) => {
                    state = DiskMapState::DiskDigit(d);
                    repeat_n('.', n).collect::<String>()
                }
            };
            v
        })
        .collect::<Vec<String>>();
    substrs.iter().fold(String::new(), |acc, s| acc + s)
}

fn expand_disk_map2(map: &str) -> String {
    let mut state = DiskMapState::DiskDigit(0);
    let substrs = map
        .chars()
        .map(|c| {
            let n = c.to_digit(10).unwrap() as usize;
            let v = match state {
                DiskMapState::DiskDigit(d) => {
                    state = DiskMapState::Empty(d + 1);
                    repeat_n(char::from_u32(d).unwrap(), n).collect::<String>()
                }
                DiskMapState::Empty(d) => {
                    state = DiskMapState::DiskDigit(d);
                    repeat_n(char::MAX, n).collect::<String>()
                }
            };
            v
        })
        .collect::<Vec<String>>();
    substrs.iter().fold(String::new(), |acc, s| acc + s)
}

fn sort_expanded_disk_map(map: &str) -> String {
    let (mut lix, mut rix): (usize, usize) = (0, map.len() - 1);
    let mut output: Vec<char> = map.chars().collect();
    while rix > lix {
        if output[rix] == char::MAX {
            // If under right index is a `.`, do nothing, proceed to left.
            rix -= 1;
            continue;
        }
        if output[lix] != char::MAX {
            // If under left index not a `.`, do nothing, proceed to right.
            lix += 1;
            continue;
        }
        output[lix] = output[rix];
        output[rix] = char::MAX;
    }
    output.iter().collect::<String>()
}
fn sort2_expanded_disk_map(map: &str) -> String {
    let (mut lix, mut rix): (usize, usize) = (0, (map.chars().count() - 1));
    let mut output: Vec<char> = map.chars().collect();
    while rix > lix {
        if output[rix] == char::MAX {
            // If under right index is a `.`, do nothing, proceed to left.
            output.pop();
            rix -= 1;
            continue;
        }
        if output[lix] != char::MAX {
            // If under left index not a `.`, do nothing, proceed to right.
            lix += 1;
            continue;
        }
        output[lix] = output[rix];
        output.pop();
        rix -= 1;
    }
    output.iter().collect::<String>()
}

fn checksum(map: &str) -> u64 {
    map.chars().enumerate().fold(0, |acc, (i, c)| {
        if c == char::MAX {
            acc
        } else {
            acc + (i as u64) * (c.to_digit(10).ok_or_else(|| 0u64).unwrap() as u64)
        }
    })
}

fn checksum2(map: &str) -> u64 {
    map.chars().enumerate().fold(0, |acc, (i, c)| {
        if c == char::MAX {
            acc
        } else {
            acc + (i as u64) * ((c as u32) as u64)
        }
    })
}

pub fn part01(map: &str) -> u64 {
    let exp_map = expand_disk_map2(map);
    println!("{:?}", exp_map);
    let sorted_map = sort2_expanded_disk_map(exp_map.as_str());
    println!("{:?}", sorted_map);
    checksum2(sorted_map.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_disk_map() {
        assert_eq!(
            expand_disk_map("2333133121414131402"),
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        )
    }
    #[test]
    fn test_expand_disk_map2() {
        assert_eq!(expand_disk_map2("233313312141413140202"), "\0\0\u{10ffff}\u{10ffff}\u{10ffff}\u{1}\u{1}\u{1}\u{10ffff}\u{10ffff}\u{10ffff}\u{2}\u{10ffff}\u{10ffff}\u{10ffff}\u{3}\u{3}\u{3}\u{10ffff}\u{4}\u{4}\u{10ffff}\u{5}\u{5}\u{5}\u{5}\u{10ffff}\u{6}\u{6}\u{6}\u{6}\u{10ffff}\u{7}\u{7}\u{7}\u{10ffff}\u{8}\u{8}\u{8}\u{8}\t\t\n\n".to_string())
    }
    #[test]
    fn test_sort_expanded_disk_map() {
        assert_eq!(
            sort_expanded_disk_map("00...111...2...333.44.5555.6666.777.888899"),
            "0099811188827773336446555566..............".to_string()
        )
    }

    #[test]
    fn test_sort2_expanded_disk_map() {
        assert_eq!(
            sort2_expanded_disk_map("00...111...2...333.44.5555.6666.777.888899"),
            "0099811188827773336446555566".to_string()
        )
    }

    #[test]
    fn test_sort_expanded_disk_map2() {
        assert_eq!(sort_expanded_disk_map("\0\0\u{10ffff}\u{10ffff}\u{10ffff}\u{1}\u{1}\u{1}\u{10ffff}\u{10ffff}\u{10ffff}\u{2}\u{10ffff}\u{10ffff}\u{10ffff}\u{3}\u{3}\u{3}\u{10ffff}\u{4}\u{4}\u{10ffff}\u{5}\u{5}\u{5}\u{5}\u{10ffff}\u{6}\u{6}\u{6}\u{6}\u{10ffff}\u{7}\u{7}\u{7}\u{10ffff}\u{8}\u{8}\u{8}\u{8}\t\t\n\n"), "\0\0\n\n\t\u{1}\u{1}\u{1}\t\u{8}\u{8}\u{2}\u{8}\u{8}\u{7}\u{3}\u{3}\u{3}\u{7}\u{4}\u{4}\u{7}\u{5}\u{5}\u{5}\u{5}\u{6}\u{6}\u{6}\u{6}..............".to_string())
    }

    #[test]
    fn test_sort2_expanded_disk_map2() {
        assert_eq!(sort2_expanded_disk_map("\0\0\u{10ffff}\u{10ffff}\u{10ffff}\u{1}\u{1}\u{1}\u{10ffff}\u{10ffff}\u{10ffff}\u{2}\u{10ffff}\u{10ffff}\u{10ffff}\u{3}\u{3}\u{3}\u{10ffff}\u{4}\u{4}\u{10ffff}\u{5}\u{5}\u{5}\u{5}\u{10ffff}\u{6}\u{6}\u{6}\u{6}\u{10ffff}\u{7}\u{7}\u{7}\u{10ffff}\u{8}\u{8}\u{8}\u{8}\t\t\n\n"), "\0\0\n\n\t\u{1}\u{1}\u{1}\t\u{8}\u{8}\u{2}\u{8}\u{8}\u{7}\u{3}\u{3}\u{3}\u{7}\u{4}\u{4}\u{7}\u{5}\u{5}\u{5}\u{5}\u{6}\u{6}\u{6}\u{6}".to_string())
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("0099811188827773336446555566"), 1928);
    }

    #[test]
    fn test_checksum2() {
        assert_eq!(checksum2("\0\0\n\n\t\u{1}\u{1}\u{1}\t\u{8}\u{8}\u{2}\u{8}\u{8}\u{7}\u{3}\u{3}\u{3}\u{7}\u{4}\u{4}\u{7}\u{5}\u{5}\u{5}\u{5}\u{6}\u{6}\u{6}\u{6}"), 2351);
    }
}
