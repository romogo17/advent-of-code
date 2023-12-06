use day_05::*;

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let almanac = Almanac::new_from_aoc_input(input);

    almanac
        .seeds
        .iter()
        .map(|seed| {
            let soil = almanac.seed_to_soil(*seed);
            let fertilizer = almanac.soil_to_fertilizer(soil);
            let water = almanac.fertilizer_to_water(fertilizer);
            let light = almanac.water_to_light(water);
            let temperature = almanac.light_to_temperature(light);
            let humidity = almanac.temperature_to_humidity(temperature);
            let location = almanac.humidity_to_location(humidity);

            // println!("Seed {seed} => Soil {soil} => Fertilizer {fertilizer} => Water {water} => Light {light} => Temperature {temperature} => Humidity {humidity} => Location {location}");

            location
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod day_05_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = process(input);
        assert_eq!(output, 35);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 1181555926);
    }
}
