mod constants;
mod processor;
mod reader;
mod tacview;
mod writer;

use std::sync::Arc;
use std::{process, thread};

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
    let (header, body) = processor::split_into_header_and_body(lines)?;

    let coalition_per_line = Arc::new(processor::divide_body_by_coalition(&body)?);

    let header = Arc::new(header);
    let body = Arc::new(body);
    let input_filename = Arc::new(input_filename);

    let handles: Vec<_> = vec![Coalition::Blue, Coalition::Red, Coalition::Purple]
        .into_iter()
        .map(|c| {
            let z = is_zip;
            let b = body.clone();
            let coalitions = coalition_per_line.clone();
            let h = header.clone();
            let i = input_filename.clone();
            thread::spawn(move || {
                let mut writer = writer::create_writer(z, &*i.clone(), &c).unwrap();
                writer.write_strings(&*h).unwrap();
                writer.write_for_coalition(&*b, &*coalitions, c).unwrap();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
