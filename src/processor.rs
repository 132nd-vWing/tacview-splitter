use std::hash::Hash;

use crate::constants::{COMMENT, MINUS};
use crate::tacview::{Coalition, CoalitionIDs};

pub fn split_into_header_and_body<S: AsRef<str>>(lines: &[S]) -> (&[S], &[S]) {
    let mut i = 0;
    for line in lines {
        if line.as_ref().chars().next().expect("malformed line") == COMMENT {
            break;
        }
        i += 1;
    }
    (&lines[..i], &lines[i..])
}

pub fn divide_body_by_coalition<S: AsRef<str> + Hash + Eq>(body: &[S]) -> Vec<Coalition> {
    let mut result = vec![];
    let mut coalition_ids: CoalitionIDs = CoalitionIDs::new();

    let mut old_line = Line::default();

    for line_s in body {
        let processed = Line::process_line(old_line, line_s, &mut coalition_ids);
        result.push(processed.coalition.clone());
        old_line = processed;
    }
    result
}

struct Line {
    line_type: LineType,
    continued: bool,
    coalition: Coalition,
}

impl Line {
    fn process_line<'a, S: AsRef<str>>(
        old_line: Line,
        current_line: &'a S,
        coalition_ids: &mut CoalitionIDs<'a>,
    ) -> Self {
        let line_type = match old_line.continued {
            true => old_line.line_type,
            false => LineType::find_type(current_line),
        };

        match line_type {
            LineType::Telemetry => Line::from_content(line_type, current_line, coalition_ids),
            LineType::Timestamp => Line::new(line_type, false, Coalition::All),
            LineType::Destruction => Line::from_content(line_type, current_line, coalition_ids),
            LineType::Unknown => panic!(""),
        }
    }

    fn from_content<'a, S: AsRef<str>>(
        line_type: LineType,
        current_line: &'a S,
        coalition_ids: &mut CoalitionIDs<'a>,
    ) -> Self {
        let id = Self::get_id_from_line(current_line);
        let continued = Self::will_line_continue(current_line);
        let coalition = Self::assign_id_to_coalitions(coalition_ids, current_line, id);
        Line::new(line_type, continued, coalition)
    }

    fn assign_id_to_coalitions<'a, S: AsRef<str>>(
        coalition_ids: &mut CoalitionIDs<'a>,
        line: &S,
        id: &'a str,
    ) -> Coalition {
        if Coalition::line_contains_coalition(line) {
            let coalition = Coalition::from_line(line);
            coalition_ids.insert(id, coalition.clone());
            coalition
        } else {
            Coalition::Unknown
        }
    }

    fn will_line_continue<S: AsRef<str>>(current_line: &S) -> bool {
        current_line.as_ref().ends_with('\\')
    }

    fn get_id_from_line<S: AsRef<str>>(line: &S) -> &str {
        line.as_ref().split_once(',').unwrap().0
    }

    fn new(line_type: LineType, continued: bool, coalition: Coalition) -> Self {
        Self {
            line_type,
            continued,
            coalition,
        }
    }

    fn default() -> Self {
        Self::new(LineType::Unknown, false, Coalition::Unknown)
    }
}

#[derive(PartialEq, Clone)]
enum LineType {
    Unknown,
    Timestamp,
    Destruction,
    Telemetry,
}

impl LineType {
    fn find_type<S: AsRef<str>>(line: &S) -> Self {
        let first_char = line.as_ref().chars().next().unwrap();
        if first_char == COMMENT {
            Self::Timestamp
        } else if first_char == MINUS {
            Self::Destruction
        } else {
            LineType::Telemetry
        }
    }
}
