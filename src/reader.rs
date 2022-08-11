use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn find_input_file() -> Result<(String, bool), Error> {
    let read_dir = fs::read_dir(".")?;

    for entry_result in read_dir {
        let path_buf = entry_result?.path();
        let filename = path_buf.to_string_lossy().to_string();

        if filename.ends_with(EXTENSION_TXT) {
            return Ok((filename, false));
        } else if filename.ends_with(EXTENSION_ZIP) {
            return Ok((filename, true));
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "No tacview input file found in current directory.",
    ))
}

pub fn read_data(filename: &str, is_zip: bool) -> Result<Vec<String>, Error> {
    let file = fs::File::open(filename)?;
    let buf = BufReader::new(file);
    if is_zip {
        Ok(read_zip(buf)?)
    } else {
        Ok(read_txt(buf)?)
    }
}

fn read_zip(buf: BufReader<File>) -> Result<Vec<String>, Error> {
    let mut archive = zip::ZipArchive::new(buf)?;
    let inner_file = archive.by_index(0)?;
    let inner_buf = BufReader::new(inner_file);
    Ok(read_txt(inner_buf)?)
}

fn read_txt<T: BufRead>(buf: T) -> Result<Vec<String>, Error> {
    let lines: Vec<String> = buf.lines().map(|l|).collect();
    Ok(lines)
}
