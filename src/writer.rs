use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};
use crate::tacview::Coalition;
use std::error::Error as ErrTrait;
use std::fs::File;
use std::io::{Error, Write};
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn create_writer(
    is_zip: bool,
    filename: &str,
    coalition: &Coalition,
) -> Result<Box<dyn Write>, Box<dyn ErrTrait>> {
    let base_name = remove_extension(filename, is_zip);
    println!("writing {coalition}");
    if is_zip {
        Ok(Box::new(create_zipwriter(
            &format!("{base_name}_{coalition}{EXTENSION_ZIP}"),
            &format!("{base_name}_{coalition}{EXTENSION_TXT}"),
        )?))
    } else {
        Ok(Box::new(create_textwriter(base_name)?))
    }
}

fn remove_extension(filename: &str, is_zip: bool) -> &str {
    if is_zip {
        &filename[..(filename.len() - EXTENSION_ZIP.len())]
    } else {
        &filename[..(filename.len() - EXTENSION_TXT.len())]
    }
}

fn create_textwriter(filename: &str) -> Result<File, Error> {
    File::create(filename)
}

fn create_zipwriter(outer_name: &str, inner_name: &str) -> Result<ZipWriter<File>, Error> {
    let mut writer = ZipWriter::new(create_textwriter(outer_name)?);
    writer.start_file(
        inner_name,
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )?;
    Ok(writer)
}

pub trait StringWriter {
    fn write_line<S: AsRef<str>>(&mut self, line: &S) -> Result<(), Error>;

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<(), Error>;

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: Coalition,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl<T> StringWriter for T
where
    T: Write,
{
    fn write_line<S: AsRef<str>>(&mut self, line: &S) -> Result<(), Error> {
        writeln!(self, "{}", line.as_ref())
    }

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<(), Error> {
        for line in lines {
            self.write_line(line)?;
        }
        Ok(())
    }

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: Coalition,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(lines.len(), coalition_per_line.len());

        let iter = lines
            .iter()
            .zip(coalition_per_line)
            .filter(|(_, c)| **c == coalition || **c == Coalition::All)
            .map(|(l, _)| l);

        for line in iter {
            self.write_line(line)?;
        }

        Ok(())
    }
}
