use cached::proc_macro::cached;
use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    IResult,
};
use tracing::{debug, info, warn};

const NUM_KEYPAD: [(char, IVec2); 11] = [
    ('7', IVec2::new(0, 0)),
    ('8', IVec2::new(1, 0)),
    ('9', IVec2::new(2, 0)),
    ('4', IVec2::new(0, 1)),
    ('5', IVec2::new(1, 1)),
    ('6', IVec2::new(2, 1)),
    ('1', IVec2::new(0, 2)),
    ('2', IVec2::new(1, 2)),
    ('3', IVec2::new(2, 2)),
    ('0', IVec2::new(1, 3)),
    ('A', IVec2::new(2, 3)),
];

fn num_pos(num: char) -> Option<IVec2> {
    match NUM_KEYPAD.iter().find(|(c, _)| *c == num) {
        Some((_, pos)) => Some(*pos),
        None => None,
    }
}

const DIR_KEYPAD: [(char, IVec2); 5] = [
    ('^', IVec2::new(1, 0)),
    ('A', IVec2::new(2, 0)),
    ('<', IVec2::new(0, 1)),
    ('v', IVec2::new(1, 1)),
    ('>', IVec2::new(2, 1)),
];

fn dir_pos(dir: char) -> Option<IVec2> {
    match DIR_KEYPAD.iter().find(|(c, _)| *c == dir) {
        Some((_, pos)) => Some(*pos),
        None => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Keypad {
    Num,
    Dir,
}

impl Keypad {
    fn is_pos_valid(&self, pos: &IVec2) -> bool {
        match self {
            Keypad::Num => {
                pos.x >= 0 && pos.x < 3 && pos.y >= 0 && pos.y < 4 && pos != &IVec2::new(0, 3)
            }
            Keypad::Dir => {
                pos.x >= 0 && pos.x < 3 && pos.y >= 0 && pos.y < 2 && pos != &IVec2::new(0, 0)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
    fn to_delta(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, codes) = separated_list1(line_ending, alphanumeric1)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, codes))
}

fn moves_to_key(start_pos: IVec2, c: char, keypad: &Keypad) -> Vec<String> {
    let end_pos = match keypad {
        Keypad::Num => num_pos(c).expect("to have a valid num key"),
        Keypad::Dir => dir_pos(c).expect("to have a valid dir key"),
    };
    let delta = end_pos - start_pos;

    let mut moves = Vec::new();
    if delta.x > 0 {
        moves.extend(std::iter::repeat(Direction::Right).take(delta.x as usize));
    }
    if delta.x < 0 {
        moves.extend(std::iter::repeat(Direction::Left).take(-delta.x as usize));
    }
    if delta.y > 0 {
        moves.extend(std::iter::repeat(Direction::Down).take(delta.y as usize));
    }
    if delta.y < 0 {
        moves.extend(std::iter::repeat(Direction::Up).take(-delta.y as usize));
    }

    debug!("Moves required to reach key '{}': {:?}", c, moves);

    let res: Vec<_> = moves
        .iter()
        .permutations(moves.len())
        .filter_map(|move_perm| {
            let mut pos = start_pos;
            let mut moves = String::new();
            for m in move_perm.iter() {
                pos += m.to_delta();
                if !keypad.is_pos_valid(&pos) {
                    return None;
                }
                moves.push(m.to_char());
            }
            moves.push('A');
            Some(moves)
        })
        .collect();

    debug!("{} valid moves to reach key '{}': {:?}", res.len(), c, res);
    res
}

#[cached(key = "String", convert = r#"{ format!("{}_{}", code, depth) }"#)]
fn keycode(code: &str, depth: i32, keypad: &Keypad) -> i128 {
    if depth == 0 {
        return code.len() as i128;
    }

    let mut res = 0;
    let mut curr_pos = match keypad {
        Keypad::Num => num_pos('A').expect("to have a valid num start"),
        Keypad::Dir => dir_pos('A').expect("to have a valid dir start"),
    };

    for c in code.chars() {
        info!("In pos {} going to '{}' (depth {})", curr_pos, c, depth);
        let mut c_cost = i128::MAX;
        for moves in moves_to_key(curr_pos, c, &keypad) {
            let cost = keycode(moves.as_str(), depth - 1, &Keypad::Dir);
            c_cost = c_cost.min(cost);
        }
        res += c_cost;
        curr_pos = match keypad {
            Keypad::Num => num_pos(c).expect("to have a valid num key"),
            Keypad::Dir => dir_pos(c).expect("to have a valid dir key"),
        };
    }

    res
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i128> {
    let (_input, codes) = all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?codes);

    let result: i128 = codes
        .iter()
        .map(|code| {
            let result = keycode(code, 26, &Keypad::Num);
            let nums = code[..3].parse::<i128>().unwrap();

            debug!("Code: {}, result: {}, nums: {}", code, result, nums);

            result * nums
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod day_21_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!(154115708116294, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(116821732384052, process(input)?);
        Ok(())
    }
}
