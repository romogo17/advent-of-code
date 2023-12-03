fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u32 {
    input.split('\n').map(|line| process_line(line)).sum()
}

fn process_line(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|d| d.to_digit(10)).collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    [first, last].iter().fold(0, |acc, &x| acc * 10 + x)
}

#[cfg(test)]
mod day_01_part1 {
    use super::*;

    #[test]
    fn example() {
        let output = process(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(output, 142);
    }

    #[test]
    fn input1() {
        let output = process(include_str!("../../inputs/input1.txt"));
        assert_eq!(output, 53651);
    }
}
