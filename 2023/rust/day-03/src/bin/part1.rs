use day_03::*;

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u32 {
    let engine_schematic = EngineSchematic::new(input);
    println!("Engine schematic is \n{}", engine_schematic);

    engine_schematic
        .part_numbers()
        .iter()
        .map(|num_in_engine| num_in_engine.value)
        .sum()
}

#[cfg(test)]
mod day_03_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = process(input);
        assert_eq!(output, 4361);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");

        let output = process(input);
        assert_eq!(output, 550064);
    }
}
