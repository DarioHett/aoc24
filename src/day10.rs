#[allow(dead_code)]
use itertools::Itertools;

const POSSIBLE_MOVES: &[(i32, i32)] = &[
    (-1, 0), // Left
    (0, -1), // Up
    (1, 0),  // Right
    (0, 1),  //  Down
];

pub fn eligible_locations_closure<'a>(
    (max_x, max_y): (&'a i32, &'a i32),
) -> (impl Fn((i32, i32)) -> Vec<(i32, i32)> + use<'a>) {
    let eligible_locations = |(x, y): (i32, i32)| {
        POSSIBLE_MOVES
            .into_iter()
            .map(|(h, v)| (h + x, v + y))
            .filter(|(a, b)| (0 <= *a) && (*a <= *max_x) && (0 <= *b) && (*b <= *max_y))
            .collect()
    };
    eligible_locations
}
fn eligible_locations((x, y): (i32, i32), (max_x, max_y): (&i32, &i32)) -> Vec<(i32, i32)> {
    POSSIBLE_MOVES
        .into_iter()
        .map(|(h, v)| (h + x, v + y))
        .filter(|(a, b)| (0 <= *a) && (*a <= *max_x) && (0 <= *b) && (*b <= *max_y))
        .collect()
}

fn value_at_location(location: (i32, i32), map: &Vec<Vec<i32>>) -> i32 {
    map[location.1 as usize][location.0 as usize]
}
#[allow(dead_code)]
fn value_at_location_is_lower(
    current_value: i32,
    location: (i32, i32),
    map: &Vec<Vec<i32>>,
) -> bool {
    current_value > value_at_location(location, map)
}
#[allow(dead_code)]
fn value_at_location_is_higher(
    current_value: i32,
    location: (i32, i32),
    map: &Vec<Vec<i32>>,
) -> bool {
    !value_at_location_is_lower(current_value + 1, location, map)
}

pub fn value_at_location_is_increment(
    current_value: i32,
    location: (i32, i32),
    map: &Vec<Vec<i32>>,
) -> bool {
    (current_value + 1) == value_at_location(location, map)
}

pub fn is_peak(location: (i32, i32), map: &Vec<Vec<i32>>) -> bool {
    value_at_location(location, map) == 9
}

pub fn is_trailhead(location: (i32, i32), map: &Vec<Vec<i32>>) -> bool {
    value_at_location(location, map) == 0
}

pub fn parse(line: &str) -> Vec<i32> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

pub fn recursion(
    curr_loc: (i32, i32),
    map: &Vec<Vec<i32>>,
    bounds: (&i32, &i32),
) -> Vec<(i32, i32)> {
    if is_peak(curr_loc, map) {
        return vec![curr_loc];
    }
    let cv = value_at_location(curr_loc, map);
    let locs = eligible_locations(curr_loc, bounds)
        .iter()
        .cloned()
        .filter(|loc| value_at_location_is_increment(cv, *loc, &map))
        .collect::<Vec<(i32, i32)>>();
    let mut collection = Vec::new();
    for loc in locs {
        let v = recursion(loc, &map, bounds)
            .into_iter()
            .unique()
            .collect::<Vec<(i32, i32)>>();
        collection.push(v);
    }
    collection
        .into_iter()
        .flatten()
        .collect::<Vec<(i32, i32)>>()
}

pub fn recursion2(
    curr_loc: (i32, i32),
    map: &Vec<Vec<i32>>,
    bounds: (&i32, &i32),
) -> Vec<(i32, i32)> {
    if is_peak(curr_loc, map) {
        return vec![curr_loc];
    }
    let cv = value_at_location(curr_loc, map);
    let locs = eligible_locations(curr_loc, bounds)
        .iter()
        .cloned()
        .filter(|loc| value_at_location_is_increment(cv, *loc, &map))
        .collect::<Vec<(i32, i32)>>();
    let mut collection = Vec::new();
    for loc in locs {
        let v = crate::day10::recursion2(loc, &map, bounds)
            .into_iter()
            .collect::<Vec<(i32, i32)>>();
        collection.push(v);
    }
    collection
        .into_iter()
        .flatten()
        .collect::<Vec<(i32, i32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = "01\n10";
        let map = lines.lines().map(|s| parse(s)).collect::<Vec<Vec<i32>>>();
        assert_eq!(map, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test_eligible_moves() {
        let eligible_locations = eligible_locations_closure((&5, &6));
        let i = eligible_locations((0, 0)).iter().count();
        assert_eq!(i, 2);
    }

    #[test]
    fn test_value_at_location_is_higher() {
        let map: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let eligible_locations = eligible_locations_closure((&1, &1));
        let v = eligible_locations((0, 0))
            .into_iter()
            .map(|loc| value_at_location_is_higher(0, loc, &map))
            .reduce(|acc, loc| acc && loc)
            .unwrap();
        assert_eq!(v, true);
    }
    #[test]
    fn test_value_at_location_is_higher2() {
        let map: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let eligible_locations = eligible_locations_closure((&1, &1));
        let v = eligible_locations((0, 0))
            .into_iter()
            .map(|loc| value_at_location_is_higher(1, loc, &map))
            .reduce(|acc, loc| acc && loc)
            .unwrap();
        assert_eq!(v, false);
    }
}
