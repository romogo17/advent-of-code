fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u32 {
    input.split('\n').map(|line| process_line(line)).sum()
}

fn process_line(line: &str) -> u32 {
    let digits: Vec<u32> = extract_digits(line);
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    [first, last].iter().fold(0, |acc, &x| acc * 10 + x)
}

fn extract_digits(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        let digit_elem = match line.get(i..) {
            Some(substring) if substring.starts_with("one") => 1,
            Some(substring) if substring.starts_with("two") => 2,
            Some(substring) if substring.starts_with("three") => 3,
            Some(substring) if substring.starts_with("four") => 4,
            Some(substring) if substring.starts_with("five") => 5,
            Some(substring) if substring.starts_with("six") => 6,
            Some(substring) if substring.starts_with("seven") => 7,
            Some(substring) if substring.starts_with("eight") => 8,
            Some(substring) if substring.starts_with("nine") => 9,
            Some(digit) if digit.chars().nth(0).unwrap().is_digit(10) => {
                digit.chars().nth(0).unwrap().to_digit(10).unwrap()
            }
            Some(_) => continue,
            None => break,
        };
        digits.push(digit_elem);
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let output = process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(output, 281);
    }

    #[test]
    fn part2_input1() {
        let output = process(include_str!("../../inputs/input1.txt"));
        assert_eq!(output, 53894);
    }
}
