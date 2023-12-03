#[path = "../lib/mod.rs"]
mod lib;

use lib::cubes::*;

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
