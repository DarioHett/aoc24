pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub enum Day02State {
    Initial,
    FirstStep(i32),
    Increasing(i32),
    Decreasing(i32),
    Unsafe,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
pub enum Dampened {
    Dampened,
    Undampened,
}
pub fn day02_is_safe(acc: Day02State, v: i32) -> Day02State {
    match acc {
        Day02State::Unsafe => Day02State::Unsafe,
        Day02State::Initial => return Day02State::FirstStep(v),
        Day02State::FirstStep(x) => match v - x {
            1 => Day02State::Increasing(v),
            2 => Day02State::Increasing(v),
            3 => Day02State::Increasing(v),
            -1 => Day02State::Decreasing(v),
            -2 => Day02State::Decreasing(v),
            -3 => Day02State::Decreasing(v),
            _ => Day02State::Unsafe,
        },
        Day02State::Increasing(x) => match v - x {
            1 => Day02State::Increasing(v),
            2 => Day02State::Increasing(v),
            3 => Day02State::Increasing(v),
            _ => Day02State::Unsafe,
        },
        Day02State::Decreasing(x) => match v - x {
            -1 => Day02State::Decreasing(v),
            -2 => Day02State::Decreasing(v),
            -3 => Day02State::Decreasing(v),
            _ => Day02State::Unsafe,
        },
    }
}

pub fn dampened_day02_is_safe(
    (accl, accr): (Day02State, Dampened),
    v: i32,
) -> (Day02State, Dampened) {
    let new_state = day02_is_safe(accl.clone(), v);
    match new_state {
        Day02State::Unsafe => match accr {
            Dampened::Dampened => (Day02State::Unsafe, accr),
            Dampened::Undampened => (accl, Dampened::Dampened),
        },
        _ => (new_state, accr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
