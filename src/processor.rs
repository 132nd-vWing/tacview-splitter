use anyhow::{bail, Context, Result};

use crate::tacview::{Coalition, CoalitionIDs};

const COMMENT: char = '#';
const MINUS: char = '-';
const ZERO: char = '0';
const UTF8_BOM: char = '\u{FEFF}'; // byte order mark. a String can begin with that, in which case
                                   // we need to discard it.

pub fn split_into_header_and_body<S>(mut lines: Vec<S>) -> Result<(Vec<S>, Vec<S>)>
where
    S: AsRef<str> + From<String>,
{
    let mut split_idx = 0;
    for line in lines.iter() {
        if line
            .as_ref()
            .chars()
            .find(|c| *c != UTF8_BOM)
            .with_context(|| "found an empty line")?
            == COMMENT
        {
            break;
        }
        split_idx += 1;
    }
    let body = lines.split_off(split_idx);
    Ok((lines, body))
}

pub fn get_coalition_per_line<S>(body: &[S]) -> Result<Vec<Coalition>>
where
    S: AsRef<str>,
{
    let mut result = vec![];
    let mut coalition_ids: CoalitionIDs = CoalitionIDs::new();

    let mut old_line = Line::default();

    for line_s in body {
        let processed = Line::process_line(old_line, line_s, &mut coalition_ids)?;
        result.push(processed.coalition.clone());
        old_line = processed;
    }
    sanity_check(&result, body);
    Ok(result)
}

fn sanity_check<S, T>(v1: &[S], v2: &[T]) {
    if v1.len() != v2.len() {
        println!("Warning: data is inconsistent");
        println!("{}", v1.len());
        println!("{}", v2.len());
    }
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
    ) -> Result<Self> {
        let line_type = match old_line.continued {
            true => old_line.line_type,
            false => LineType::find_type(current_line)
                .with_context(|| "could not get line type for this line: {current_line}")?,
        };

        match line_type {
            LineType::ArbitraryData => Ok(Line::new(
                line_type,
                Self::will_line_continue(current_line),
                Coalition::All,
            )),
            LineType::Telemetry => Ok(Line::from_content(line_type, current_line, coalition_ids)
                .with_context(|| "could not parse telemetry line")?),
            LineType::Timestamp => Ok(Line::new(line_type, false, Coalition::All)),
            LineType::Destruction => Ok(Line::from_content(line_type, current_line, coalition_ids)
                .with_context(|| {
                    format!(
                        "could not parse this destruction line: {}",
                        current_line.as_ref()
                    )
                })?),
            LineType::Unknown => bail!("unknown line type"),
        }
    }

    fn from_content<'a, S: AsRef<str>>(
        line_type: LineType,
        current_line: &'a S,
        coalition_ids: &mut CoalitionIDs<'a>,
    ) -> Result<Self> {
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

    fn get_id_from_line<'a, S>(line: &'a S, line_type: &LineType) -> Result<&'a str>
    where
        S: AsRef<str>,
    {
        let local_line = line.as_ref();
        let result = if line_type == &LineType::Destruction {
            &local_line[1..local_line.len()]
        } else {
            local_line
                .split_once(',')
                .with_context(|| format!("cannot get ID from line {local_line}"))?
                .0
        };
        if result.is_empty() {
            bail!("cannot get ID from this line\n{local_line}")
        } else {
            Ok(result)
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

#[derive(PartialEq, Clone, Debug)]
enum LineType {
    Unknown,
    Timestamp,
    Destruction,
    Telemetry,
    ArbitraryData,
}

impl LineType {
    fn find_type<S: AsRef<str>>(line: &S) -> Result<Self> {
        let first_char = line
            .as_ref()
            .chars()
            .find(|c| *c != UTF8_BOM)
            .with_context(|| "line in tacview file is empty")?;
        let mut buffer: [u8; 4] = [0; 4];
        first_char.encode_utf8(&mut buffer);
        if first_char == COMMENT {
            Ok(Self::Timestamp)
        } else if first_char == MINUS {
            Ok(Self::Destruction)
        } else if first_char == ZERO {
            Ok(LineType::ArbitraryData)
        } else if first_char.is_whitespace() {
            bail!("found line beginning with whitespace")
        } else {
            Ok(LineType::Telemetry)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Line, LineType};

    #[test]
    fn test_find_type() {
        assert_eq!(LineType::find_type(&"#0").unwrap(), LineType::Timestamp);
        assert_eq!(
            LineType::find_type(&"102,T=5.0785362|6.203").unwrap(),
            LineType::Telemetry
        );
        assert_eq!(LineType::find_type(&"-102").unwrap(), LineType::Destruction);
        assert_eq!(
            LineType::find_type(&"0,Authentication").unwrap(),
            LineType::ArbitraryData
        );
    }

    #[test]
    #[should_panic]
    fn test_find_type_for_space() {
        LineType::find_type(&" ").unwrap();
    }

    #[test]
    fn test_get_id_from_line() {
        let lines = vec![
            ("102,T=5.0785362|6.203", "102"),
            ("2d02,T=", "2d02"),
            ("248b02,T=6.70283", "248b02"),
        ];
        for l in lines {
            assert_eq!(
                &Line::get_id_from_line(&l.0, &LineType::Telemetry).unwrap(),
                &l.1
            )
        }
        let lines = vec![("-102", "102"), ("-2d02", "2d02"), ("-248b02", "248b02")];
        for l in lines {
            assert_eq!(
                &Line::get_id_from_line(&l.0, &LineType::Destruction).unwrap(),
                &l.1
            )
        }
    }
}
