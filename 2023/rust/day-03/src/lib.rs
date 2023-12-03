use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EngineSchematic {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl EngineSchematic {
    pub fn new(input_str: &str) -> EngineSchematic {
        let lines: Vec<&str> = input_str.split('\n').collect();

        let mut data: Vec<char> = Vec::with_capacity(lines.len() * lines[0].len());
        let rows = lines.len();
        let cols = lines[0].len();

        lines.iter().for_each(|line| {
            data.extend_from_slice(line.chars().collect::<Vec<char>>().as_slice());
        });

        EngineSchematic { data, rows, cols }
    }

    fn num_at(&self, start: usize, len: usize) -> u32 {
        self.data
            .iter()
            .skip(start)
            .take(len)
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }

    pub fn part_numbers(&self) -> Vec<NumInEngine> {
        let engine_schematic = self.clone();

        engine_schematic
            .into_iter()
            .filter(|num_in_engine| self.is_part_number(num_in_engine))
            .collect()
    }

    pub fn gears(&self) -> HashMap<(i32, i32), Vec<NumInEngine>> {
        let part_numbers = self.part_numbers();

        let mut gears: HashMap<(i32, i32), Vec<NumInEngine>> = HashMap::new();

        for num_in_engine in part_numbers {
            let col_idx =
                num_in_engine.col as i32..num_in_engine.col as i32 + num_in_engine.len as i32;

            for i in (num_in_engine.row as i32 - 1)..=(num_in_engine.row as i32 + 1) {
                for j in (num_in_engine.col as i32 - 1)
                    ..(num_in_engine.col as i32 + num_in_engine.len as i32 + 1)
                {
                    if !(i == num_in_engine.row as i32 && col_idx.contains(&j))
                        && (0 <= i && i < self.rows as i32)
                        && (0 <= j && j < self.cols as i32)
                    {
                        let idx = i * self.cols as i32 + j;
                        let current_char = self.data[idx as usize];

                        if current_char == '*' {
                            gears
                                .entry((i, j))
                                .or_insert(Vec::new())
                                .push(num_in_engine.clone());
                        }
                    }
                }
            }
        }

        gears.retain(|_, v| v.len() == 2);
        gears
    }

    fn is_part_number(&self, num_in_engine: &NumInEngine) -> bool {
        let mut flag = false;
        let col_idx = num_in_engine.col as i32..num_in_engine.col as i32 + num_in_engine.len as i32;

        for i in (num_in_engine.row as i32 - 1)..=(num_in_engine.row as i32 + 1) {
            for j in (num_in_engine.col as i32 - 1)
                ..(num_in_engine.col as i32 + num_in_engine.len as i32 + 1)
            {
                if !(i == num_in_engine.row as i32 && col_idx.contains(&j))
                    && (0 <= i && i < self.rows as i32)
                    && (0 <= j && j < self.cols as i32)
                {
                    let idx = i * self.cols as i32 + j;
                    let current_char = self.data[idx as usize];

                    if !current_char.is_digit(10) && current_char != '.' {
                        flag = true;
                    }
                }
            }
        }
        flag
    }
}

impl std::fmt::Display for EngineSchematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::with_capacity(self.data.len() + self.rows);

        self.data.chunks(self.cols).for_each(|row| {
            output.push_str(row.iter().collect::<String>().as_str());
            output.push('\n');
        });

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub struct NumInEngine {
    pub value: u32,
    pub row: usize,
    pub col: usize,
    pub len: usize,
}

impl IntoIterator for EngineSchematic {
    type Item = NumInEngine;
    type IntoIter = NumInEngineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        NumInEngineIntoIterator {
            engine_schematic: self,
            row: 0,
            col: 0,
            found_digits: false,
        }
    }
}

pub struct NumInEngineIntoIterator {
    engine_schematic: EngineSchematic,
    row: usize,
    col: usize,
    found_digits: bool,
}

impl Iterator for NumInEngineIntoIterator {
    type Item = NumInEngine;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.row..self.engine_schematic.rows {
            for j in self.col..self.engine_schematic.cols {
                let idx = i * self.engine_schematic.cols + j;

                if self.engine_schematic.data[idx].is_digit(10) {
                    if !self.found_digits {
                        self.found_digits = true;
                        self.row = i;
                        self.col = j;
                    }
                    if self.found_digits && j == self.engine_schematic.cols - 1 {
                        let start = self.row * self.engine_schematic.cols + self.col;
                        let len = (i - self.row) * self.engine_schematic.cols + (j - self.col) + 1;

                        let num_in_engine = NumInEngine {
                            value: self.engine_schematic.num_at(start, len),
                            row: self.row,
                            col: self.col,
                            len,
                        };

                        self.found_digits = false;
                        self.row = i + 1;
                        self.col = 0;

                        return Some(num_in_engine);
                    }
                } else {
                    if self.found_digits {
                        let start = self.row * self.engine_schematic.cols + self.col;
                        let len = (i - self.row) * self.engine_schematic.cols + (j - self.col);

                        let num_in_engine = NumInEngine {
                            value: self.engine_schematic.num_at(start, len),
                            row: self.row,
                            col: self.col,
                            len,
                        };

                        self.found_digits = false;
                        self.row = i;
                        self.col = j + 1; // we already know j is not a digit, we should not check it twice

                        return Some(num_in_engine);
                    }
                }
            }
            self.col = 0;
            self.found_digits = false;
        }
        None
    }
}
