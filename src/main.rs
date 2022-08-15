mod constants;
mod processor;
mod reader;
mod tacview;
mod writer;

use anyhow::Result;

fn main() -> Result<()> {
    let (input_filename, is_zip) = reader::find_input_file()?;
    println!("Processing {}", input_filename);

    let lines = reader::read_data(&input_filename, is_zip)?;

    let (header, body) = processor::split_into_header_and_body(lines)?;
    let coalition_per_line = processor::divide_body_by_coalition(&body)?;

    let output_data =
        writer::OutputData::new(input_filename, is_zip, header, body, coalition_per_line);
    output_data.save_to_disk()?;

    Ok(())
}
