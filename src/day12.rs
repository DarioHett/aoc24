use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Mul;

pub fn parse_line(input: &str, line_number: usize) -> Vec<(char, usize, usize)> {
    input
        .chars()
        .enumerate()
        .map(|(x, c)| (c, x, line_number))
        .collect::<Vec<_>>()
}

pub fn eligible_locations_closure<'a>(
    (max_x, max_y): (&'a usize, &'a usize),
) -> (impl Fn((usize, usize)) -> Vec<(usize, usize)> + use<'a>) {
    let eligible_locations = |(x, y): (usize, usize)| {
        let mut v = vec![];
        if x < *max_x {
            v.push((x + 1, y))
        }
        if y < *max_y {
            v.push((x, y + 1))
        }
        if y > 0 {
            v.push((x, y - 1))
        }
        if x > 0 {
            v.push((x - 1, y))
        }
        v
    };
    eligible_locations
}

pub fn grid_to_map(
    grid: &Vec<Vec<(char, usize, usize)>>,
) -> HashMap<(char, usize, usize), Vec<(char, usize, usize)>> {
    let binding = grid[0].len() - 1;
    let binding2 = grid.len() - 1;
    let eligible_locations = eligible_locations_closure((&binding, &binding2));
    let mut map: HashMap<(char, usize, usize), Vec<(char, usize, usize)>> = HashMap::new();
    grid.iter().for_each(|vec| {
        vec.iter().for_each(|(c, x, y)| {
            let locs = eligible_locations((*x, *y));
            let assoc_cells = locs
                .iter()
                .filter(|(x1, y1)| grid[*y1][*x1].0 == *c)
                .map(|(x1, y1)| grid[*y1][*x1])
                .collect();
            map.insert((*c, *x, *y), assoc_cells);
        })
    });
    map
}

pub fn walk(
    current: (char, usize, usize),
    walked: &Vec<(char, usize, usize)>,
    map: &HashMap<(char, usize, usize), Vec<(char, usize, usize)>>,
) -> Vec<(char, usize, usize)> {
    let mut new_walked = walked.clone();
    new_walked.push(current);

    let next_possible_steps = map
        .get(&current)
        .unwrap()
        .iter()
        .filter(|&v| !new_walked.clone().iter().contains(v))
        .collect::<Vec<_>>();
    if next_possible_steps.len() > 0 {
        let mut walk_collection = Vec::new();
        for curr in map
            .get(&current)
            .unwrap()
            .iter()
            .filter(|&v| !new_walked.clone().iter().contains(v))
        {
            walk_collection.push(walk(*curr, &new_walked, map));
        }
        walk_collection.push(vec![current]);
        return walk_collection.into_iter().flatten().unique().collect();
    } else {
        return vec![current];
    }
}

pub fn component(
    start: (char, usize, usize),
    map: &HashMap<(char, usize, usize), Vec<(char, usize, usize)>>,
) -> Vec<(char, usize, usize)> {
    let mut visited_nodes = HashSet::new();
    visited_nodes.insert(start);
    let mut next_possible_steps = map
        .get(&start)
        .unwrap()
        .iter()
        .cloned()
        .filter(|v| !visited_nodes.contains(v))
        .collect::<Vec<(char, usize, usize)>>();
    while let Some(step) = next_possible_steps.pop() {
        visited_nodes.insert(step);
        let new_possible_steps = map
            .get(&step)
            .unwrap()
            .clone()
            .iter()
            .cloned()
            .filter(|&v| !visited_nodes.contains(&v))
            .collect::<Vec<(char, usize, usize)>>();
        for ele in new_possible_steps {
            next_possible_steps.push(ele);
        }
    }
    visited_nodes
        .iter()
        .cloned()
        .sorted()
        .dedup()
        .collect::<Vec<(char, usize, usize)>>()
}

pub fn components(
    map: &HashMap<(char, usize, usize), Vec<(char, usize, usize)>>,
) -> HashSet<Vec<(char, usize, usize)>> {
    let mut cs = HashSet::new();
    for k in map.keys() {
        if cs.iter().map(|c: &Vec<_>| c.contains(k)).any(|b| b) {
            continue;
        }
        let cmpt = component(*k, &map);
        cs.insert(cmpt);
    }
    cs
}

pub fn components_with_sizes(
    map: &HashMap<(char, usize, usize), Vec<(char, usize, usize)>>,
    size_map: &HashMap<(char, usize, usize), usize>,
) -> Vec<usize> {
    let mut cs = HashSet::new();
    for k in map.keys() {
        if cs.iter().map(|c: &Vec<_>| c.contains(k)).any(|b| b) {
            continue;
        }
        let cmpt = component(*k, &map);
        cs.insert(cmpt);
    }
    cs.into_iter()
        .map(|c| component_fencing(&c, size_map).mul(c.len()))
        .collect()
}

pub fn component_sizes(cs: &HashSet<Vec<(char, usize, usize)>>) -> Vec<usize> {
    cs.iter().map(|c| c.len()).collect::<Vec<_>>()
}

pub fn size_map(
    map: &HashMap<(char, usize, usize), Vec<(char, usize, usize)>>,
) -> HashMap<(char, usize, usize), usize> {
    let mut size_map = HashMap::new();
    let size_pairs = map.iter().map(|(x, y)| (*x, y.len())).collect::<Vec<_>>();
    for (k, v) in size_pairs {
        size_map.insert(k, 4 - v);
    }
    size_map
}

pub fn component_fencing(
    c: &Vec<(char, usize, usize)>,
    size_map: &HashMap<(char, usize, usize), usize>,
) -> usize {
    c.iter().map(|x| size_map.get(&x).unwrap()).sum()
}
//  `other` is above.
fn is_above(plant: &(char, usize, usize), other: &(char, usize, usize)) -> bool {
    //  `other` is above.
    (plant.1 == other.1) && (other.2 + 1 == plant.2)
}
fn is_beneath(plant: &(char, usize, usize), other: &(char, usize, usize)) -> bool {
    (plant.1 == other.1) && (other.2 == plant.2 + 1)
}
fn is_left(plant: &(char, usize, usize), other: &(char, usize, usize)) -> bool {
    //  `other` is left.
    (plant.1 == other.1 + 1) && (other.2 == plant.2)
}

fn is_right(plant: &(char, usize, usize), other: &(char, usize, usize)) -> bool {
    (plant.1 + 1 == other.1) && (other.2 == plant.2)
}

fn has_right_neighbor(plant: &(char, usize, usize), others: &Vec<(char, usize, usize)>) -> bool {
    others.into_iter().map(|o| is_right(plant, o)).any(|b| b)
}

fn has_left_neighbor(plant: &(char, usize, usize), others: &Vec<(char, usize, usize)>) -> bool {
    others.into_iter().map(|o| is_left(plant, o)).any(|b| b)
}

fn has_above_neighbor(plant: &(char, usize, usize), others: &Vec<(char, usize, usize)>) -> bool {
    others.into_iter().map(|o| is_above(plant, o)).any(|b| b)
}

fn has_beneath_neighbor(plant: &(char, usize, usize), others: &Vec<(char, usize, usize)>) -> bool {
    others.into_iter().map(|o| is_beneath(plant, o)).any(|b| b)
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum Walls {
    Above,
    Beneath,
    Left,
    Right,
}
pub fn plant_in_component_walls(
    plant: &(char, usize, usize),
    component: &Vec<(char, usize, usize)>,
) -> Vec<Walls> {
    let mut walls = Vec::new();

    if !has_beneath_neighbor(plant, component) {
        walls.push(Walls::Beneath);
    }
    if !has_above_neighbor(plant, component) {
        walls.push(Walls::Above);
    }
    if !has_left_neighbor(plant, component) {
        walls.push(Walls::Left);
    }
    if !has_right_neighbor(plant, component) {
        walls.push(Walls::Right);
    }
    walls
}

pub fn component_walls(
    component: &Vec<(char, usize, usize)>,
) -> HashMap<Walls, Vec<(char, usize, usize)>> {
    let mut walls: HashMap<Walls, Vec<(char, usize, usize)>> = HashMap::new();
    walls.insert(Walls::Above, Vec::new());
    walls.insert(Walls::Beneath, Vec::new());
    walls.insert(Walls::Left, Vec::new());
    walls.insert(Walls::Right, Vec::new());
    for plant in component {
        let plant_walls = plant_in_component_walls(plant, component);
        for wall in plant_walls {
            let v = walls.get_mut(&wall).unwrap();
            v.push(*plant)
        }
    }
    walls
}

pub fn vertical_sides(sides: &Vec<(char, usize, usize)>) -> usize {
    let mut ctr = 0;
    // let xs = sides.iter().map(|(c,x,y)| x).dedup().collect::<Vec<(usize)>>();
    let s: Vec<_> = sides
        .iter()
        .sorted_by_key(|(_, x, _)| x)
        .chunk_by(|(_, x, _)| x)
        .into_iter()
        .map(|(_, z)| z.sorted_by_key(|(_, _, y)| y).collect::<Vec<_>>())
        .collect();
    // println!("vert={:?}", sides);
    ctr += s.len();
    for grp in s.iter() {
        grp.iter().tuple_windows().for_each(|(a, b)| {
            if (b.2 - a.2) > 1 {
                ctr += 1
            }
        })
    }
    ctr
}

pub fn horizontal_sides(sides: &Vec<(char, usize, usize)>) -> usize {
    let mut ctr = 0;
    // let xs = sides.iter().map(|(c,x,y)| x).dedup().collect::<Vec<(usize)>>();
    let s: Vec<_> = sides
        .iter()
        .sorted_by_key(|(_, _, x)| x)
        .chunk_by(|(_, _, x)| x)
        .into_iter()
        .map(|(_, z)| z.sorted_by_key(|(_, y, _)| y).collect::<Vec<_>>())
        .collect();
    // println!("horiz={:?}", sides);

    ctr += s.len();
    for grp in s.iter() {
        // println!("{:?}", grp);
        grp.iter().tuple_windows().for_each(|(a, b)| {
            // println!("{:?} {:?}", a,b);
            if (b.1 - a.1) > 1 {
                ctr += 1
            }
        })
    }
    ctr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "ABC";
        let stones = parse_line(line, 1);
        assert_eq!(stones, [('A', 0, 1), ('B', 1, 1), ('C', 2, 1)]);
    }

    #[test]
    fn test_eligible_locations() {
        let eligible_locations = eligible_locations_closure((&2, &2));

        assert_eq!(eligible_locations((0, 0)), vec![(1, 0), (0, 1)]);
    }

    #[test]
    fn test_grid_to_map() {
        let input = "ADD\nAAD";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        assert_eq!(map.get(&('A', 0, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&('A', 0, 0)).unwrap().len(), 1);
        assert_eq!(map.get(&('A', 1, 1)).unwrap().len(), 1);

        assert_eq!(map.get(&('D', 1, 0)).unwrap().len(), 1);
        assert_eq!(map.get(&('D', 2, 0)).unwrap().len(), 2);
        assert_eq!(map.get(&('D', 2, 1)).unwrap().len(), 1);
    }
    #[test]
    fn test_component() {
        let input = "ADD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cmpt = component(('A', 0, 0), &map);
        // println!("{:?}", cmpt);
        assert_eq!(cmpt.len(), 3);
        assert_eq!(cmpt, vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)]);
    }

    #[test]
    fn test_components() {
        let input = "AAD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cs = components(&map);
        assert_eq!(cs.len(), 3);
        // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    }

    #[test]
    fn test_plant_walls() {
        let input = "AAD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cs = components(&map);
        let c = cs.iter().next().unwrap();
        println!("{:?}", c);

        println!("{:?}", plant_in_component_walls(&c[0], c));
        // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    }

    #[test]
    fn test_component_walls() {
        let input = "AAD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cs = components(&map);
        let c = cs.iter().next().unwrap();
        println!("{:?}", component_walls(&c));
        // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    }

    #[test]
    fn test_component_wall_counts() {
        let input = "AAD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        let cs = components(&map);
        let c = cs.iter().next().unwrap();
        let cwall_map = component_walls(&c);
        let mut ctr = 0;
        cwall_map.iter().for_each(|(w, ps)| match w {
            Walls::Above => ctr += horizontal_sides(ps),
            Walls::Beneath => ctr += horizontal_sides(ps),
            Walls::Left => ctr += vertical_sides(ps),
            Walls::Right => ctr += vertical_sides(ps),
        });
        println!("{:?}", cwall_map);
        println!("{:?}", ctr);
        // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    }
    // #[test]
    // fn test_component_sizes() {
    //     let input = "AAD\nAAD\nCCC";
    //     let grid = input
    //         .lines()
    //         .enumerate()
    //         .map(|(i, l)| parse_line(l, i))
    //         .collect::<Vec<_>>();
    //     let map = grid_to_map(&grid);
    //     let cs = components(&map);
    //     assert_eq!(component_sizes(&cs), vec![2, 3, 4]);
    //     // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    // }
    // #[test]
    // fn test_components_with_sizes() {
    //     let input = "AAD\nAAD\nCCC";
    //     let grid = input
    //         .lines()
    //         .enumerate()
    //         .map(|(i, l)| parse_line(l, i))
    //         .collect::<Vec<_>>();
    //     let map = grid_to_map(&grid);
    //     let size_map = size_map(&map);
    //     let cs = components_with_sizes(&map, &size_map);
    //     println!("{:?}", cs);
    //     assert_eq!(cs, vec![24, 32, 12]);
    //     // assert_eq!(cs, vec![vec![('D', 1, 0), ('D', 2, 0), ('D', 2, 1)], vec![('A', 0, 0), ('A', 0, 1), ('A', 1, 1)], vec![('C', 0, 2), ('C', 1, 2), ('C', 2, 2)]]);
    // }

    #[test]
    fn test_vertical_sides() {
        let sides = vec![('D', 2usize, 0usize), ('D', 4, 0), ('D', 2, 3), ('D', 2, 1)];
        let mut ctr = 0;
        // let xs = sides.iter().map(|(c,x,y)| x).dedup().collect::<Vec<(usize)>>();
        let s: Vec<_> = sides
            .iter()
            .sorted_by_key(|(_, x, _)| x)
            .chunk_by(|(_, x, _)| x)
            .into_iter()
            .map(|(_i, z)| z.sorted_by_key(|(_, _, y)| y).collect::<Vec<_>>())
            .collect();
        println!("{:?}", s);
        ctr += s.len();
        for grp in s.iter() {
            grp.iter().tuple_windows().for_each(|(a, b)| {
                if (b.2 - a.2) > 1 {
                    ctr += 1
                }
            })
        }
        println!("{:?}", ctr);
    }

    #[test]
    fn test_walk() {
        let input = "ADD\nAAD\nCCC";
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| parse_line(l, i))
            .collect::<Vec<_>>();
        let map = grid_to_map(&grid);
        for (i, j) in (0usize..3usize).cartesian_product(0..3) {
            let item = grid[i][j];
            let walked = Vec::new();
            let new_walked = walk(item, &walked, &map);
            println!(
                "{:?}, {:?}",
                item,
                new_walked.iter().sorted().collect::<Vec<_>>()
            );
        }
    }
    //
    // #[test]
    // fn test_print_map() {
    //     let input = "ADD\nAAD\nCCC";
    //     let grid = input.lines().enumerate().map(|(i, l)| parse_line(l, i)).collect::<Vec<_>>();
    //     let map = grid_to_map(&grid);
    //     let mut size_map = HashMap::new();
    //     let size_pairs = map.iter().map(|(x, y)| (*x, y.len())).collect::<Vec<_>>();
    //     for (k, v) in size_pairs {
    //         size_map.insert(k, 4-v);
    //     }
    //     println!("{:?}", size_map);
    //     let mut all_walks = HashSet::new();
    //     let walked = Vec::new();
    //     let to_be_walked = (0..grid.len()).cartesian_product(0..grid[0].len());
    //     let mut has_been_walked: Vec::<(usize, usize)> = Vec::new();
    //     for (i,j) in to_be_walked {
    //         if has_been_walked.contains(&(i,j)) {
    //             continue;
    //         };
    //         let item = grid[i][j];
    //         let other_walked = walk(item, &walked, &map).iter().cloned().sorted().unique().collect::<Vec<_>>().clone();
    //         for (_,a,b) in &other_walked {
    //             has_been_walked.push((*a,*b));
    //         }
    //         all_walks.insert(other_walked);
    //     }
    //     let binding = all_walks.clone();
    //     let unique_walks = binding.iter().dedup().collect::<Vec<_>>();
    //     println!("{:?}", unique_walks);
    //     println!("{:?}", unique_walks.into_iter().map(|v| v.iter().map(|w| size_map.get(w).unwrap().clone()).collect::<Vec<_>>().iter().sum::<usize>()).collect::<Vec<_>>().iter().sum::<usize>())
    //     }
}
