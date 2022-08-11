use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};
use crate::tacview::Coalition;
use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn create_writer(is_zip: bool, filename: &str) -> Box<dyn Write> {
    let base_name = remove_extension(filename, is_zip);
    if is_zip {
        Box::new(create_zipwriter(
            &format!("{base_name}{EXTENSION_ZIP}"),
            &format!("{base_name}{EXTENSION_TXT}"),
        ))
    } else {
        Box::new(create_textwriter(base_name))
    }
}

fn remove_extension(filename: &str, is_zip: bool) -> &str {
    if is_zip {
        &filename[..(filename.len() - EXTENSION_ZIP.len())]
    } else {
        &filename[..(filename.len() - EXTENSION_TXT.len())]
    }
}

fn create_textwriter(filename: &str) -> File {
    File::create(filename).unwrap()
}

fn create_zipwriter(outer_name: &str, inner_name: &str) -> ZipWriter<File> {
    let mut writer = ZipWriter::new(create_textwriter(outer_name));
    writer
        .start_file(
            inner_name,
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
        )
        .unwrap();
    writer
}
pub trait StringWriter {
    fn write_line<S: AsRef<str>>(&mut self, line: &S);

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]);

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: Coalition,
    );
}

impl<T> StringWriter for T
where
    T: Write,
{
    fn write_line<S: AsRef<str>>(&mut self, line: &S) {
        writeln!(self, "{}", line.as_ref()).unwrap();
    }

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]) {
        for line in lines {
            self.write_line(line);
        }
    }

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: Coalition,
    ) {
        assert_eq!(lines.len(), coalition_per_line.len());
        lines
            .iter()
            .zip(coalition_per_line)
            .filter(|(_, c)| **c == coalition || **c == Coalition::All)
            .for_each(|(l, _)| self.write_line(l));
    }
}
