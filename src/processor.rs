use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

use crate::constants::{COMMENT, MINUS};
use crate::tacview::{Coalition, CoalitionIDs};

pub fn split_into_header_and_body<S>(lines: &[S]) -> Result<(&[S], &[S]), ProcessingError>
where
    S: AsRef<str>,
{
    let mut i = 0;
    for line in lines {
        if line
            .as_ref()
            .chars()
            .next()
            .ok_or(ProcessingError::CannotSplitIntoHeaderAndBody)?
            == COMMENT
        {
            break;
        }
        i += 1;
    }
    Ok((&lines[..i], &lines[i..]))
}

pub fn divide_body_by_coalition<S>(body: &[S]) -> Result<Vec<Coalition>, Box<dyn Error>>
where
    S: AsRef<str> + Hash + Eq,
{
    let mut result = vec![];
    let mut coalition_ids: CoalitionIDs = CoalitionIDs::new();

    let mut old_line = Line::default();

    for line_s in body {
        let processed = Line::process_line(old_line, line_s, &mut coalition_ids)?;
        result.push(processed.coalition.clone());
        old_line = processed;
    }
    Ok(result)
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
    ) -> Result<Self, ProcessingError> {
        let line_type = match old_line.continued {
            true => old_line.line_type,
            false => LineType::find_type(current_line)?,
        };

        match line_type {
            LineType::Telemetry => Ok(Line::from_content(line_type, current_line, coalition_ids)?),
            LineType::Timestamp => Ok(Line::new(line_type, false, Coalition::All)),
            LineType::Destruction => {
                Ok(Line::from_content(line_type, current_line, coalition_ids)?)
            }
            LineType::Unknown => Err(ProcessingError::UnknownLineType),
        }
    }

    fn from_content<'a, S: AsRef<str>>(
        line_type: LineType,
        current_line: &'a S,
        coalition_ids: &mut CoalitionIDs<'a>,
    ) -> Result<Self, ProcessingError> {
        let id = Self::get_id_from_line(current_line)?;
        let continued = Self::will_line_continue(current_line);
        let coalition = Self::assign_id_to_coalitions(coalition_ids, current_line, id);
        Ok(Line::new(line_type, continued, coalition))
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

    fn get_id_from_line<S: AsRef<str>>(line: &S) -> Result<&str, ProcessingError> {
        Ok(line
            .as_ref()
            .split_once(',')
            .ok_or(ProcessingError::CannotGetIDFromLine)?
            .0)
    }

    fn new(line_type: LineType, continued: bool, coalition: Coalition) -> Self {
        Self {
            line_type,
            continued,
            coalition,
        }
    }
}

impl Default for Line {
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
    fn find_type<S: AsRef<str>>(line: &S) -> Result<Self, ProcessingError> {
        let first_char = line
            .as_ref()
            .chars()
            .next()
            .ok_or(ProcessingError::LineIsEmptyError)?;
        if first_char == COMMENT {
            Ok(Self::Timestamp)
        } else if first_char == MINUS {
            Ok(Self::Destruction)
        } else {
            Ok(LineType::Telemetry)
        }
    }
}

#[derive(Debug)]
pub enum ProcessingError {
    LineIsEmptyError,
    UnknownLineType,
    CannotSplitIntoHeaderAndBody,
    CannotGetIDFromLine,
}

impl Error for ProcessingError {}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineIsEmptyError => write!(f, "line in tacview file is empty"),
            Self::UnknownLineType => write!(f, "unknown line type in tacview file"),
            Self::CannotSplitIntoHeaderAndBody => {
                write!(f, "cannot split the file into header and body")
            }
            Self::CannotGetIDFromLine => write!(f, "cannot get unit ID from line"),
        }
    }
}
