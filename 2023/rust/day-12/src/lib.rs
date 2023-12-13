use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::Operational => write!(f, "."),
            Condition::Damaged => write!(f, "#"),
            Condition::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpringsLine {
    conditions: Vec<Condition>,
}

impl SpringsLine {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }

    pub fn from_missing_permutation(template: &SpringsLine, permutation: &Vec<&Condition>) -> Self {
        let mut permutation_iter = permutation.iter();
        let conditions: Vec<Condition> = template
            .conditions
            .iter()
            .map(|condition| match condition {
                Condition::Unknown => {
                    let next_perm = permutation_iter
                        .next()
                        .expect("permutation should be long enough");
                    **next_perm
                }
                _ => condition.clone(),
            })
            .collect();

        Self { conditions }
    }

    pub fn count_missing(&self) -> usize {
        self.conditions
            .iter()
            .filter(|condition| matches!(condition, Condition::Unknown))
            .count()
    }

    pub fn count_damaged_group_lengths(&self) -> Vec<u64> {
        let mut groups: Vec<u64> = vec![0];
        let mut prev = &Condition::Unknown;

        for condition in self.conditions.iter() {
            let groups_len = groups.len();
            match (condition, prev) {
                (Condition::Damaged, _) => {
                    groups[groups_len - 1] += 1;
                }
                (Condition::Operational, Condition::Damaged) => {
                    groups.push(0);
                }
                _ => {}
            }
            prev = condition
        }

        groups.iter().filter(|g| **g > 0).copied().collect()
    }
}

impl fmt::Display for SpringsLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.conditions {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}
