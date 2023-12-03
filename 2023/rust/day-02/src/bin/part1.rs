#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct CubeCount {
    color: CubeColor,
    count: u32,
}

impl CubeCount {
    fn new(color: CubeColor, count: u32) -> CubeCount {
        CubeCount {
            color: color,
            count: count,
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    cube_counts: Vec<CubeCount>,
}

impl CubeSet {
    fn new_from_rgb(red: u32, green: u32, blue: u32) -> CubeSet {
        CubeSet {
            cube_counts: vec![
                CubeCount::new(CubeColor::Red, red),
                CubeCount::new(CubeColor::Green, green),
                CubeCount::new(CubeColor::Blue, blue),
            ],
        }
    }

    fn default() -> CubeSet {
        CubeSet {
            cube_counts: vec![
                CubeCount::new(CubeColor::Red, 0),
                CubeCount::new(CubeColor::Green, 0),
                CubeCount::new(CubeColor::Blue, 0),
            ],
        }
    }

    fn add_cubes(&mut self, color: CubeColor, count: u32) {
        match color {
            CubeColor::Red => {
                self.cube_counts
                    .iter_mut()
                    .find(|cube_count| cube_count.color == CubeColor::Red)
                    .unwrap()
                    .count += count
            }
            CubeColor::Green => {
                self.cube_counts
                    .iter_mut()
                    .find(|cube_count| cube_count.color == CubeColor::Green)
                    .unwrap()
                    .count += count
            }
            CubeColor::Blue => {
                self.cube_counts
                    .iter_mut()
                    .find(|cube_count| cube_count.color == CubeColor::Blue)
                    .unwrap()
                    .count += count
            }
        }
    }

    fn can_be_created_from(&self, other: &CubeSet) -> bool {
        self.cube_counts
            .iter()
            .map(|cube_count| {
                let other_count = other
                    .cube_counts
                    .iter()
                    .find(|other_cube_count| other_cube_count.color == cube_count.color)
                    .unwrap();

                (cube_count.count, other_count.count)
            })
            .all(|(self_count, other_count)| self_count <= other_count)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn new_from_aoc_input(input: &str) -> Game {
        let v: Vec<&str> = input.split(": ").collect();

        Game {
            id: v[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            cube_sets: v[1]
                .split("; ")
                .map(|cube_set_str| {
                    let mut cube_set = CubeSet::default();

                    for cube_count_str in cube_set_str.split(", ") {
                        let cube_count: Vec<&str> = cube_count_str.split(" ").collect();

                        let count = cube_count[0].parse::<u32>().unwrap();
                        let color = match cube_count[1] {
                            "red" => CubeColor::Red,
                            "green" => CubeColor::Green,
                            "blue" => CubeColor::Blue,
                            _ => panic!("Invalid color"),
                        };

                        cube_set.add_cubes(color, count);
                    }

                    cube_set
                })
                .collect(),
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");

    let games = input
        .split("\n")
        .map(|game_str| Game::new_from_aoc_input(game_str))
        .collect::<Vec<Game>>();

    let total_cubes = CubeSet::new_from_rgb(12, 13, 14);

    let output = process(games, total_cubes);
    println!("Output is {output}");
}

fn process(games: Vec<Game>, total_cubes: CubeSet) -> u32 {
    // for game in games {
    //     for (idx, cube_set) in game.cube_sets.iter().enumerate() {
    //         println!(
    //             "\n(GAME: {}) Cube set {}\nChecking cube_set {:?}",
    //             game.id, idx, cube_set
    //         );
    //         if cube_set.can_be_created_from(&total_cubes) {
    //             println!("→ Cube set {} in game {} is valid!", idx, game.id);
    //         } else {
    //             println!("→ Cube set {} in game {} is NOT valid!", idx, game.id);
    //         }
    //     }
    // }
    games
        .iter()
        .map(|game| {
            match game
                .cube_sets
                .iter()
                .map(|cube_set| cube_set.can_be_created_from(&total_cubes))
                .all(|bool_res| bool_res == true)
            {
                true => game.id,
                false => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = input
            .split("\n")
            .map(|game_str| Game::new_from_aoc_input(game_str))
            .collect::<Vec<Game>>();

        let total_cubes = CubeSet::new_from_rgb(12, 13, 14);

        let output = process(games, total_cubes);

        assert_eq!(output, 8);
    }

    #[test]
    fn part1_input1() {
        let input = include_str!("../../inputs/input1.txt");

        let games = input
            .split("\n")
            .map(|game_str| Game::new_from_aoc_input(game_str))
            .collect::<Vec<Game>>();

        let total_cubes = CubeSet::new_from_rgb(12, 13, 14);

        let output = process(games, total_cubes);

        assert_eq!(output, 2720);
    }
}
