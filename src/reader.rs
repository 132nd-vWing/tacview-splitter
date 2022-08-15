use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};

pub fn find_input_file() -> Result<(String, bool)> {
    let read_dir = fs::read_dir(".").with_context(|| "could not open the current directory")?;

    for entry_result in read_dir {
        let path_buf = entry_result
            .with_context(|| "error while iterating over directory")?
            .path();
        let filename = path_buf.to_string_lossy().to_string();

        if filename.ends_with(EXTENSION_TXT) {
            return Ok((filename, false));
        } else if filename.ends_with(EXTENSION_ZIP) {
            return Ok((filename, true));
        }
    }
    bail!("No tacview input file found in current directory.")
}

pub fn read_data(filename: &str, is_zip: bool) -> Result<Vec<String>> {
    let file = fs::File::open(filename).with_context(|| "could not open {filename}")?;
    let buf = BufReader::new(file);
    if is_zip {
        Ok(read_zip(buf)?)
    } else {
        Ok(read_txt(buf)?)
    }
}

fn read_zip(buf: BufReader<File>) -> Result<Vec<String>> {
    let mut archive = zip::ZipArchive::new(buf).with_context(|| "could not open zip archive")?;
    let inner_file = archive
        .by_index(0)
        .with_context(|| "could not get the txt file in the zip archive")?;
    let inner_buf = BufReader::new(inner_file);
    read_txt(inner_buf)
}

fn read_txt(buf: impl BufRead) -> Result<Vec<String>> {
    let mut lines = vec![];
    for line in buf.lines() {
        lines.push(line.with_context(|| "could not read line from file, is it valid UTF-8?")?);
    }
    Ok(lines)
}
