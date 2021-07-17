use std::{fs, str};
use std::io::{BufRead, BufReader};

use zip;

use tacview_splitter::lib;
use tacview_splitter::lib::Handling;

const COMMENT: char = '#';
const MINUS: char = '-';

const EXTENSION_ZIP: &str = ".zip.acmi";
const EXTENSION_TXT: &str = ".txt.acmi";

#[derive(PartialEq)]
enum LineType {
    Unknown,
    Timestamp,
    Destruction,
    Telemetry
}

fn main() {
    let (input_filename, is_zip) = find_input_file();
    println!("Processing {}", input_filename);
    let lines = read_data(&input_filename, is_zip);
    let (header, body) = split_into_header_and_body(lines);
    let bodies_by_coalition = divide_body_by_coalition(&body);

    let output_filenames = get_output_filenames(&input_filename, is_zip);
    if is_zip {
        let mut descriptors = lib::Descriptors::<zip::ZipWriter<fs::File>>::new(output_filenames);
        descriptors.write(header, bodies_by_coalition);
    } else {
        let mut descriptors = lib::Descriptors::<fs::File>::new(output_filenames);
        descriptors.write(header, bodies_by_coalition);
    }
}

fn split_into_header_and_body(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut i=0;
    for line in &lines {
        if line.chars().nth(0).expect("malformed line") == COMMENT {
            break
        }
        i += 1;
    }
    return (lines[..i].to_vec(), lines[i..].to_vec());
}

fn divide_body_by_coalition(body: &Vec<String>) -> lib::BodiesByCoalition {
    let mut bbc = lib::BodiesByCoalition{blue: Vec::new(), red: Vec::new(), violet: Vec::new()};
    let mut continued = false;
    let mut line_type = LineType::Unknown;
    let mut coalitions = lib::IDs{blue: Vec::new(), red: Vec::new(), violet: Vec::new(), unknown: Vec::new()};
    for line in body {
        let result = process_line(continued, &mut coalitions, line, line_type);
        line_type = result.0;
        continued = result.1;
        let id = result.2;
        if line_type == LineType::Timestamp {
            bbc.blue.push(line);
            bbc.red.push(line);
            bbc.violet.push(line);
        } else {  // destruction or telemetry
            if coalitions.blue.contains(&id) {
                bbc.blue.push(line);
            } else if coalitions.red.contains(&id) {
                bbc.red.push(line);
            } else if coalitions.violet.contains(&id) {
                bbc.violet.push(line);
            }
        }
    }
    bbc
}

fn process_line<'a>(continued: bool, coalitions: &mut lib::IDs<'a>, line: &'a String, last_line_type: LineType) -> (LineType, bool, &'a str) {
    let mut id = "";
    let line_type;
    if !continued {
        line_type = determine_line_type(line);
        if line_type == LineType::Telemetry {
            id = get_id_from_line(line);
            assign_id_to_coalitions(coalitions, line, id)
        }
    } else {
        line_type = last_line_type;
    }
    let line_will_continue = will_line_continue(line);
    (line_type, line_will_continue, id)
}

fn will_line_continue(line: &String) -> bool {
    let line_will_continue: bool;
    if line.ends_with("\\") {
        line_will_continue = true;
    } else {
        line_will_continue = false;
    }
    line_will_continue
}

fn get_id_from_line(line: &String) -> &str {
    let result = line.split_once(',');
    let split = match result {
        Some(t) => t,
        None => panic!("Could not get ID from line!")
    };
    let id = split.0;
    id
}

fn assign_id_to_coalitions<'a>(coalitions: &mut lib::IDs<'a>, line: &'a String, id: &'a str) {
    if line.contains("Color=") {
        if line.contains("Color=Blue") {
            coalitions.blue.push(id);
        } else if line.contains("Color=Red") {
            coalitions.red.push(id);
        } else if line.contains("Color=Violet") {
            coalitions.violet.push(id);
        } else {
            coalitions.unknown.push(id);
        }
    }
}

fn determine_line_type(line: &String) -> LineType {
    let line_type: LineType;

    let first_char = line.chars().nth(0).expect("malformed line");
    if first_char == COMMENT {
        line_type = LineType::Timestamp;
    } else if first_char == MINUS {
        line_type = LineType::Destruction;
    } else {
        line_type = LineType::Telemetry;
    }
    return line_type
}

fn get_output_filenames(input_filename: &String, is_zip: bool) -> lib::OutputFilenames {
    let output_filenames_zip: lib::FilenamesVariant;
    let output_filenames_txt: lib::FilenamesVariant;

    if is_zip {
        output_filenames_zip = get_output_filenames_for_extension(
            input_filename, EXTENSION_ZIP, EXTENSION_ZIP
        );
        output_filenames_txt = get_output_filenames_for_extension(
            input_filename, EXTENSION_ZIP, EXTENSION_TXT
        );
    } else {
        output_filenames_zip = get_output_filenames_dummy();
        output_filenames_txt = get_output_filenames_for_extension(
            input_filename, EXTENSION_TXT, EXTENSION_TXT
        );
    }

    let output_filenames = lib::OutputFilenames{txt: output_filenames_txt, zip: output_filenames_zip};
    output_filenames
}

fn get_output_filenames_dummy() -> lib::FilenamesVariant {
    let blue = "".to_string();
    let red = "".to_string();
    let violet = "".to_string();
    let output_filenames = lib::FilenamesVariant{blue, red, violet};
    output_filenames
}

fn get_output_filenames_for_extension(input_filename: &String, old_extension: &str, new_extension: &str) -> lib::FilenamesVariant {
    let blue = lib::get_output_filenames_individual(input_filename, old_extension, new_extension, "_blue");
    let red = lib::get_output_filenames_individual(input_filename, old_extension,  new_extension,"_red");
    let violet = lib::get_output_filenames_individual(input_filename, old_extension,  new_extension,"_violet");
    let output_filenames_zip = lib::FilenamesVariant{blue, red, violet};
    lib::sanity_check_output_filenames(input_filename, &output_filenames_zip);
    output_filenames_zip
}

fn find_input_file() -> (String, bool) {
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
    };
    println!("No tacview input file found in current directory.");
    std::process::exit(1);
}


fn read_data(filename: &String, is_zip: bool) -> Vec<String> {
    let file = fs::File::open(filename).expect("Could not read from input file");
    let buf = BufReader::new(file);
    return if is_zip {
        let mut archive = zip::ZipArchive::new(buf).expect("Could not read zip data");
        let inner_file = archive.by_index(0).expect("Could not read telemetry file from zip archive");
        let inner_buf = BufReader::new(inner_file);
        let lines: Vec<String> = inner_buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        lines
    } else {
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        lines
    }
}
