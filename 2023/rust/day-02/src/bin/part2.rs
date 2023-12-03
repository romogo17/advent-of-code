use day_02::*;

fn main() {
    let input = include_str!("../../inputs/input1.txt");

    let games = input
        .split("\n")
        .map(|game_str| Game::new_from_aoc_input(game_str))
        .collect::<Vec<Game>>();

    let output = process(games);
    println!("Output is {output}");
}

fn process(games: Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let mut min_cube_set = CubeSet::default();

            for cube_set in game.cube_sets.iter() {
                min_cube_set.update_with_max(cube_set);
            }

            min_cube_set.power()
        })
        .sum()
}

#[cfg(test)]
mod day_02_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = input
            .split("\n")
            .map(|game_str| Game::new_from_aoc_input(game_str))
            .collect::<Vec<Game>>();

        let output = process(games);

        assert_eq!(output, 2286);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");

        let games = input
            .split("\n")
            .map(|game_str| Game::new_from_aoc_input(game_str))
            .collect::<Vec<Game>>();

        let output = process(games);

        assert_eq!(output, 71535);
    }
}
