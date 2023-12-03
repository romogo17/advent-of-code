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
pub struct CubeSet {
    cube_counts: Vec<CubeCount>,
}

impl CubeSet {
    pub fn new_from_rgb(red: u32, green: u32, blue: u32) -> CubeSet {
        CubeSet {
            cube_counts: vec![
                CubeCount::new(CubeColor::Red, red),
                CubeCount::new(CubeColor::Green, green),
                CubeCount::new(CubeColor::Blue, blue),
            ],
        }
    }

    pub fn default() -> CubeSet {
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

    pub fn can_be_created_from(&self, other: &CubeSet) -> bool {
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

    pub fn update_with_max(&mut self, other: &CubeSet) -> () {
        for cube_count in self.cube_counts.iter_mut() {
            let other_count = other
                .cube_counts
                .iter()
                .find(|other_cube_count| other_cube_count.color == cube_count.color)
                .unwrap();

            if other_count.count > cube_count.count {
                cube_count.count = other_count.count;
            }
        }
    }

    pub fn power(&self) -> u32 {
        self.cube_counts
            .iter()
            .fold(1, |acc, cube_count| acc * cube_count.count)
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new_from_aoc_input(input: &str) -> Game {
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
