use std::cmp;
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seeds_v2: Vec<Range<u64>>,
    seed_to_soil_map: BTreeMap<u64, (u64, u64)>,
    soil_to_fertilizer_map: BTreeMap<u64, (u64, u64)>,
    fertilizer_to_water_map: BTreeMap<u64, (u64, u64)>,
    water_to_light_map: BTreeMap<u64, (u64, u64)>,
    light_to_temperature_map: BTreeMap<u64, (u64, u64)>,
    temperature_to_humidity_map: BTreeMap<u64, (u64, u64)>,
    humidity_to_location_map: BTreeMap<u64, (u64, u64)>,
}

impl Almanac {
    pub fn new_from_aoc_input(input: &str) -> Almanac {
        let sections: Vec<&str> = input.split("\n\n").collect();

        let seeds: Vec<u64> = sections[0]
            .split(": ")
            .last()
            .expect("there's no seeds")
            .split(" ")
            .map(|seed| seed.parse::<u64>().expect("couldn't parse seed"))
            .collect();

        Almanac {
            seeds,
            seeds_v2: vec![],
            seed_to_soil_map: Self::parse_map_section(sections[1]),
            soil_to_fertilizer_map: Self::parse_map_section(sections[2]),
            fertilizer_to_water_map: Self::parse_map_section(sections[3]),
            water_to_light_map: Self::parse_map_section(sections[4]),
            light_to_temperature_map: Self::parse_map_section(sections[5]),
            temperature_to_humidity_map: Self::parse_map_section(sections[6]),
            humidity_to_location_map: Self::parse_map_section(sections[7]),
        }
    }

    pub fn new_from_aoc_input_v2(input: &str) -> Almanac {
        let sections: Vec<&str> = input.split("\n\n").collect();

        let seeds: Vec<Range<u64>> = sections[0]
            .split(": ")
            .last()
            .expect("there's no seeds")
            .split(" ")
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0]
                    .parse::<u64>()
                    .expect("couldn't parse seed interval start");
                let len = chunk[1]
                    .parse::<u64>()
                    .expect("couldn't parse seed interval len");
                start..start + len
            })
            .collect();

        Almanac {
            seeds: vec![],
            seeds_v2: seeds,
            seed_to_soil_map: Self::parse_map_section(sections[1]),
            soil_to_fertilizer_map: Self::parse_map_section(sections[2]),
            fertilizer_to_water_map: Self::parse_map_section(sections[3]),
            water_to_light_map: Self::parse_map_section(sections[4]),
            light_to_temperature_map: Self::parse_map_section(sections[5]),
            temperature_to_humidity_map: Self::parse_map_section(sections[6]),
            humidity_to_location_map: Self::parse_map_section(sections[7]),
        }
    }

    fn parse_map_section(section: &str) -> BTreeMap<u64, (u64, u64)> {
        section
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split(" ");
                let dst = parts
                    .next()
                    .expect("no destination range start")
                    .parse::<u64>()
                    .expect("couldn't parse destination range start");
                let src = parts
                    .next()
                    .expect("no source range start")
                    .parse::<u64>()
                    .expect("couldn't parse source range start");
                let len = parts
                    .next()
                    .expect("no range length")
                    .parse::<u64>()
                    .expect("couldn't range length");
                (src, (dst, len))
            })
            .collect()
    }

    pub fn merge_overlapping_intervals(arr: &mut Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        arr.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut result: Vec<Vec<u64>> = Vec::new();
        result.push(arr[0].clone());

        for i in 1..arr.len() {
            let current: Vec<u64> = arr[i].clone();
            let j: usize = result.len() - 1;

            if current[0] >= result[j][0] && current[0] <= result[j][1] {
                result[j][1] = cmp::max(current[1], result[j][1]);
            } else {
                result.push(current);
            }
        }
        result
    }

    pub fn seed_to_soil(&self, key: u64) -> u64 {
        match self.seed_to_soil_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn soil_to_fertilizer(&self, key: u64) -> u64 {
        match self.soil_to_fertilizer_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn fertilizer_to_water(&self, key: u64) -> u64 {
        match self.fertilizer_to_water_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn water_to_light(&self, key: u64) -> u64 {
        match self.water_to_light_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn light_to_temperature(&self, key: u64) -> u64 {
        match self.light_to_temperature_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn temperature_to_humidity(&self, key: u64) -> u64 {
        match self.temperature_to_humidity_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }

    pub fn humidity_to_location(&self, key: u64) -> u64 {
        match self.humidity_to_location_map.range(..=key).next_back() {
            Some((src, (dst, len))) => {
                let distance = key - src;
                let result = dst + distance;
                if result > dst + len {
                    return key;
                }
                result
            }
            None => key,
        }
    }
}
