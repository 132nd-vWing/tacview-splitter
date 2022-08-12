use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

use crate::tacview::{Coalition, CoalitionIDs};

const COMMENT: char = '#';
const MINUS: char = '-';
const ZERO: char = '0';

pub fn split_into_header_and_body<S>(mut lines: Vec<S>) -> Result<(Vec<S>, Vec<S>), ProcessingError>
where
    S: AsRef<str>,
{
    let mut split_idx = 0;
    for line in &lines {
        if line
            .as_ref()
            .chars()
            .next()
            .ok_or(ProcessingError::CannotSplitIntoHeaderAndBody)?
            == COMMENT
        {
            break;
        }
        split_idx += 1;
    }
    let body = lines.split_off(split_idx);
    Ok((lines, body))
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
        println!("{}", current_line.as_ref());
        let line_type = match old_line.continued {
            true => old_line.line_type,
            false => LineType::find_type(current_line)?,
        };

        match line_type {
            LineType::ArbitraryData => Ok(Line::new(
                line_type,
                Self::will_line_continue(current_line),
                Coalition::All,
            )),
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
        let id = Self::get_id_from_line(current_line, &line_type)?;
        let continued = Self::will_line_continue(current_line);
        let coalition = Self::assign_id_to_coalitions(coalition_ids, current_line, id)
            .unwrap_or(Coalition::Unknown); // happens for the authentication line at the end of
                                            // a file that has verification enabled
        Ok(Line::new(line_type, continued, coalition))
    }

    fn assign_id_to_coalitions<'a, S: AsRef<str>>(
        coalition_ids: &mut CoalitionIDs<'a>,
        line: &S,
        id: &'a str,
    ) -> Option<Coalition> {
        if Coalition::line_contains_coalition(line) {
            let coalition = Coalition::from_line(line);
            coalition_ids.insert(id, &coalition);
            Some(coalition)
        } else {
            coalition_ids.get(id)
        }
    }

    fn will_line_continue<S: AsRef<str>>(current_line: &S) -> bool {
        current_line.as_ref().ends_with('\\')
    }

    fn get_id_from_line<'a, S>(
        line: &'a S,
        line_type: &LineType,
    ) -> Result<&'a str, ProcessingError>
    where
        S: AsRef<str>,
    {
        let local_line = line.as_ref();
        if line_type == &LineType::Destruction {
            Ok(&local_line[1..local_line.len()])
        } else {
            Ok(local_line
                .split_once(',')
                .ok_or_else(|| ProcessingError::CannotGetIDFromLine(local_line.to_owned()))?
                .0)
        }
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
    ArbitraryData,
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
        } else if first_char == ZERO {
            Ok(LineType::ArbitraryData)
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
    CannotGetIDFromLine(String),
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
            Self::CannotGetIDFromLine(l) => write!(f, "cannot get unit ID from line: \n{l}"),
        }
    }
}
