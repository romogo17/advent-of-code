use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
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

        let seed_to_soil_map: BTreeMap<u64, (u64, u64)> = sections[1]
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
            .collect();
        let soil_to_fertilizer_map: BTreeMap<u64, (u64, u64)> = sections[2]
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
            .collect();
        let fertilizer_to_water_map: BTreeMap<u64, (u64, u64)> = sections[3]
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
            .collect();
        let water_to_light_map: BTreeMap<u64, (u64, u64)> = sections[4]
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
            .collect();
        let light_to_temperature_map: BTreeMap<u64, (u64, u64)> = sections[5]
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
            .collect();
        let temperature_to_humidity_map: BTreeMap<u64, (u64, u64)> = sections[6]
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
            .collect();
        let humidity_to_location_map: BTreeMap<u64, (u64, u64)> = sections[7]
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
            .collect();

        Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
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
