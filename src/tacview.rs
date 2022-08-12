use std::collections::HashSet;

#[derive(PartialEq, Clone)]
pub enum Coalition {
    Blue,
    Red,
    Purple,
    All,
    Unknown,
}

impl Coalition {
    pub fn from_line<S: AsRef<str>>(line: &S) -> Self {
        let line = line.as_ref();
        if line.contains("Color=Blue") {
            Self::Blue
        } else if line.contains("Color=Red") {
            Self::Red
        } else if line.contains("Color=Purple") {
            Self::Purple
        } else {
            Self::Unknown
        }
    }

    pub fn line_contains_coalition<S: AsRef<str>>(line: &S) -> bool {
        line.as_ref().contains("Color=")
    }
}

pub struct CoalitionIDs<'a> {
    pub blue: HashSet<&'a str>,
    pub red: HashSet<&'a str>,
    pub purple: HashSet<&'a str>,
}

impl<'a> CoalitionIDs<'a> {
    pub fn new() -> Self {
        let (blue, red, purple) = (HashSet::new(), HashSet::new(), HashSet::new());
        Self { blue, red, purple }
    }

    pub fn insert(&mut self, id: &'a str, coalition: &Coalition) {
        match coalition {
            Coalition::Blue => self.blue.insert(id),
            Coalition::Red => self.red.insert(id),
            Coalition::Purple => self.purple.insert(id),
            Coalition::All | Coalition::Unknown => false,
        };
    }
}
