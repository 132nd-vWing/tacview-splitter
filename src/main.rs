mod constants;
mod processor;
mod reader;
mod tacview;
mod writer;

use rayon::prelude::*;
use std::process;
use std::sync::Arc;

use crate::{tacview::Coalition, writer::StringWriter};

fn main() {
    if let Err(err) = main_inner() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
    let (input_filename, is_zip) = reader::find_input_file()?;
    println!("Processing {}", input_filename);

    let lines = reader::read_data(&input_filename, is_zip)?;
    let (header, body) = processor::split_into_header_and_body(&lines)?;

    let coalition_per_line = Arc::new(processor::divide_body_by_coalition(body)?);
    let header = Arc::new(header);
    let body = Arc::new(body);

    let _: Vec<_> = vec![Coalition::Blue, Coalition::Red, Coalition::Purple]
        .into_par_iter()
        .map(|c| {
            let mut writer = writer::create_writer(is_zip, &input_filename.clone(), &c).unwrap();
            writer.write_strings(&header).unwrap();
            writer
                .write_for_coalition(&body, &coalition_per_line.clone(), c)
                .unwrap();
        })
        .collect();

    Ok(())
}
