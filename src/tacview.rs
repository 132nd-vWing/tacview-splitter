use std::collections::HashSet;
use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum Coalition {
    Blue,
    Red,
    Violet,
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
        } else if line.contains("Color=Violet") {
            Self::Violet
        } else {
            Self::Unknown
        }
    }

    pub fn line_contains_coalition<S: AsRef<str>>(line: &S) -> bool {
        line.as_ref().contains("Color=")
    }
}

impl Display for Coalition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blue => "blue".to_string(),
                Self::Red => "red".to_string(),
                Self::Violet => "violet".to_string(),
                Self::All => "all".to_string(),
                Self::Unknown => "unknown".to_string(),
            }
        )
    }
}

pub struct CoalitionIDs<'a> {
    pub blue: HashSet<&'a str>,
    pub red: HashSet<&'a str>,
    pub violet: HashSet<&'a str>,
}

impl<'a> CoalitionIDs<'a> {
    pub fn new() -> Self {
        let (blue, red, violet) = (HashSet::new(), HashSet::new(), HashSet::new());
        Self { blue, red, violet }
    }

    pub fn insert(&mut self, id: &'a str, coalition: &Coalition) {
        match coalition {
            Coalition::Blue => self.blue.insert(id),
            Coalition::Red => self.red.insert(id),
            Coalition::Violet => self.violet.insert(id),
            Coalition::All | Coalition::Unknown => false,
        };
    }

    pub fn get(&self, id: &'a str) -> Option<Coalition> {
        if self.blue.contains(id) {
            Some(Coalition::Blue)
        } else if self.red.contains(id) {
            Some(Coalition::Red)
        } else if self.violet.contains(id) {
            Some(Coalition::Violet)
        } else {
            None
        }
    }
}
