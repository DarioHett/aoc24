use std::collections::{HashMap, HashSet};

pub struct CharMapRep {
    orig_map: Vec<Vec<char>>,
    none_char: char,
    items: HashSet<char>,
    item_locations: HashMap<char, Vec<(usize, usize)>>,
    shape: (usize, usize),
    origin: (usize, usize),
}

impl CharMapRep {
    pub fn new(orig_map: Vec<Vec<char>>, none_char: char) -> CharMapRep {
        let mut new_set = HashSet::<char>::new();
        let mut new_map = HashMap::<char, Vec<(usize, usize)>>::new();
        orig_map.iter().enumerate().for_each(|(i, v)| {
            v.iter().enumerate().for_each(|(j, c)| {
                if c != &none_char {
                    new_set.insert(*c);
                    match new_map.get_mut(c) {
                        Some(v) => v.push((j, i)),
                        None => {new_map.insert(*c, vec![(j, i)]);}
                    };
                }
            })
        });
        CharMapRep {
            orig_map: orig_map.clone(),
            none_char,
            items: new_set.clone(),
            item_locations: new_map.clone(),
            origin: (0, 0),
            shape: (orig_map[0].len(), orig_map.len()),
        }
    }

    pub fn loc(&self, (x, y): (usize, usize)) -> Option<char> {
        match self.is_loc((x,y)) {
            true => Some(self.orig_map[y][x]),
            false => None,
        }
    }
    pub fn items(&self) -> &HashSet<char> {
        &self.items
    }

    pub fn item_locs(&self, item: &char) -> Option<&Vec<(usize, usize)>> {
        self.item_locations.get(item)
    }

    pub fn is_loc(&self, (x, y): (usize, usize)) -> bool {
        x < self.shape.0 && y < self.shape.1
    }
}
