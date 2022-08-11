use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub fn find_input_file() -> (String, bool) {
    let read_dir = fs::read_dir(".").expect("Could not read current directory");

    for entry_result in read_dir {
        let entry = entry_result.expect("Could not parse DirEntry");
        let path_buf = entry.path();
        let filename = path_buf.to_string_lossy().to_string();

        if filename.ends_with(EXTENSION_TXT) {
            return (filename, false);
        } else if filename.ends_with(EXTENSION_ZIP) {
            return (filename, true);
        }
    }
    println!("No tacview input file found in current directory.");
    std::process::exit(1);
}

pub fn read_data(filename: &str, is_zip: bool) -> Vec<String> {
    let file = fs::File::open(filename).expect("Could not read from input file");
    let buf = BufReader::new(file);
    if is_zip {
        read_zip(buf)
    } else {
        read_txt(buf)
    }
}

fn read_zip(buf: BufReader<File>) -> Vec<String> {
    let mut archive = zip::ZipArchive::new(buf).expect("Could not read zip data");
    let inner_file = archive
        .by_index(0)
        .expect("Could not read telemetry file from zip archive");
    let inner_buf = BufReader::new(inner_file);
    read_txt(inner_buf)
}

fn read_txt<T: BufRead>(buf: T) -> Vec<String> {
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    lines
}
