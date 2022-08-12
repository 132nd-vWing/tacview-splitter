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

    let coalitions = vec![Coalition::Blue, Coalition::Red, Coalition::Violet];

    // clippy wants us to combine both ierators into one. this is not what we want, because then
    // we would spawn a thread, join it, and only then spawn a new one.
    #[allow(clippy::needless_collect)]
    let handles: Vec<_> = coalitions
        .iter()
        .map(|coalition| {
            let z = is_zip;
            let b = body.clone();
            let cpl = coalition_per_line.clone();
            let c = coalition.clone();
            let h = header.clone();
            let i = input_filename.clone();
            thread::spawn(move || {
                let mut writer = writer::create_writer(z, &*i.clone(), &c).unwrap();
                writer.write_strings(&*h).unwrap();
                writer.write_for_coalition(&*b, &*cpl, c).unwrap();
            })
        })
        .collect();

    let _: Vec<_> = handles
        .into_iter()
        .zip(coalitions)
        .map(|(h, c)| {
            h.join()
                .unwrap_or_else(|_| println!("could not write data for {c}"))
        })
        .collect();

    Ok(())
}
