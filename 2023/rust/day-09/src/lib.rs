#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reading {
    values: Vec<i64>,
}

impl Reading {
    pub fn new(values: Vec<i64>) -> Self {
        Reading { values }
    }

    fn get_interpolations(&self) -> Vec<Vec<i64>> {
        let mut interpolations: Vec<Vec<i64>> = vec![];

        let mut current = self
            .values
            .windows(2)
            .map(|w| {
                let (a, b) = (w[0], w[1]);
                b - a
            })
            .collect::<Vec<i64>>();

        loop {
            interpolations.push(current.clone());
            current = current
                .windows(2)
                .map(|w| {
                    let (a, b) = (w[0], w[1]);
                    b - a
                })
                .collect::<Vec<i64>>();

            if current.iter().all(|n| n == &0) {
                current.push(0);
                interpolations.push(current.clone());
                break;
            }
        }
        interpolations
    }

    pub fn interpolate_next(&mut self) -> i64 {
        let mut interpolations = self.get_interpolations();
        interpolations.reverse();

        let mut prev: i64 = 0;
        for (idx, interpolation) in interpolations.iter_mut().enumerate() {
            match idx {
                0 => {
                    interpolation.push(prev);
                }
                _ => {
                    prev = prev + interpolation.last().unwrap();
                    interpolation.push(prev);
                }
            }
        }

        let next = prev + self.values.last().unwrap();
        self.values.push(next);
        next
    }

    pub fn interpolate_prev(&mut self) -> i64 {
        let mut interpolations = self.get_interpolations();
        interpolations.reverse();

        let mut prev: i64 = 0;
        for (idx, interpolation) in interpolations.iter_mut().enumerate() {
            match idx {
                0 => {
                    interpolation.insert(0, prev);
                }
                _ => {
                    prev = interpolation.first().unwrap() - prev;
                    interpolation.insert(0, prev);
                }
            }
        }

        let next = self.values.first().unwrap() - prev;
        self.values.insert(0, next);
        next
    }
}
