mod constants;
mod processor;
mod reader;
mod tacview;
mod writer;

use crate::{tacview::Coalition, writer::StringWriter};

fn main() {
    let (input_filename, is_zip) = reader::find_input_file();
    println!("Processing {}", input_filename);
    let lines = reader::read_data(&input_filename, is_zip);
    let (header, body) = processor::split_into_header_and_body(&lines);

    let coalition_per_line = processor::divide_body_by_coalition(body);

    let mut blue_writer = writer::create_writer(is_zip, &input_filename);
    blue_writer.write_strings(header);
    blue_writer.write_for_coalition(body, &coalition_per_line, Coalition::Blue);

    let mut red_writer = writer::create_writer(is_zip, &input_filename);
    red_writer.write_strings(header);
    red_writer.write_for_coalition(body, &coalition_per_line, Coalition::Red);

    let mut purple_writer = writer::create_writer(is_zip, &input_filename);
    purple_writer.write_strings(header);
    purple_writer.write_for_coalition(body, &coalition_per_line, Coalition::Purple);
}
