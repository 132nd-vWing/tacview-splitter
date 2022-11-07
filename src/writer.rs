use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;

use anyhow::{Context, Result};
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::constants::{EXTENSION_TXT, EXTENSION_ZIP};
use crate::tacview::Coalition;

pub fn create_writer(
    is_zip: bool,
    filename: &str,
    coalition: &Coalition,
) -> Result<Box<dyn Write>> {
    let base_name = remove_extension(filename, is_zip);
    let txt_name = format!("{base_name}_{coalition}{EXTENSION_TXT}");

    println!("writing {coalition}");
    if is_zip {
        let zip_name = format!("{base_name}_{coalition}{EXTENSION_ZIP}");
        Ok(Box::new(create_zipwriter(&zip_name, &txt_name)?))
    } else {
        Ok(Box::new(create_textwriter(&txt_name)?))
    }
}

fn remove_extension(filename: &str, is_zip: bool) -> &str {
    if is_zip {
        &filename[..(filename.len() - EXTENSION_ZIP.len())]
    } else {
        &filename[..(filename.len() - EXTENSION_TXT.len())]
    }
}

fn create_textwriter(filename: &str) -> Result<File> {
    File::create(filename).with_context(|| "could not create text file")
}

fn create_zipwriter(outer_name: &str, inner_name: &str) -> Result<ZipWriter<File>> {
    let mut writer = ZipWriter::new(create_textwriter(outer_name)?);
    writer.start_file(
        inner_name,
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )?;
    Ok(writer)
}

pub trait StringWriter {
    fn write_line<S: AsRef<str>>(&mut self, line: &S) -> Result<()>;

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<()>;

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: &Coalition,
    ) -> Result<()>;
}

impl<T> StringWriter for T
where
    T: Write,
{
    fn write_line<S: AsRef<str>>(&mut self, line: &S) -> Result<()> {
        writeln!(self, "{}", line.as_ref()).with_context(|| "could not write line")
    }

    fn write_strings<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<()> {
        for line in lines {
            self.write_line(line)?;
        }
        Ok(())
    }

    fn write_for_coalition<S: AsRef<str>>(
        &mut self,
        lines: &[S],
        coalition_per_line: &[Coalition],
        coalition: &Coalition,
    ) -> Result<()> {
        assert_eq!(lines.len(), coalition_per_line.len());

        let iter = lines
            .iter()
            .zip(coalition_per_line)
            .filter(|(_, c)| *c == coalition || **c == Coalition::All)
            .map(|(l, _)| l);

        for line in iter {
            self.write_line(line)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct OutputData {
    input_filename: Arc<String>,
    is_zip: bool,
    header: Arc<Vec<String>>,
    body: Arc<Vec<String>>,
    coalition_per_line: Arc<Vec<Coalition>>,
}

impl OutputData {
    pub fn new(
        input_filename: String,
        is_zip: bool,
        header: Vec<String>,
        body: Vec<String>,
        coalition_per_line: Vec<Coalition>,
    ) -> Self {
        let body = Arc::new(body);
        let coalition_per_line = Arc::new(coalition_per_line);
        let header = Arc::new(header);
        let input_filename = Arc::new(input_filename);

        Self {
            is_zip,
            body,
            coalition_per_line,
            header,
            input_filename,
        }
    }

    // clippy wants us to combine both ierators into one. this is not what we want, because then
    // we would spawn a thread, join it, and only then spawn a new one.
    #[allow(clippy::needless_collect)]
    pub fn save_to_disk(&self) -> Result<()> {
        let coalitions = vec![Coalition::Blue, Coalition::Red, Coalition::Violet];

        let handles: Vec<_> = coalitions
            .iter()
            .map(|coalition| {
                let thread_self = self.clone();
                let thread_coalition = coalition.clone();

                thread::spawn(move || {
                    let mut writer = create_writer(
                        thread_self.is_zip,
                        &thread_self.input_filename,
                        &thread_coalition,
                    )
                    .with_context(|| format!("could not create writer for {thread_coalition}"))
                    .unwrap();
                    writer
                        .write_strings(&thread_self.header)
                        .with_context(|| format!("could not write header for {thread_coalition}"))
                        .unwrap();
                    writer
                        .write_for_coalition(
                            &thread_self.body,
                            &thread_self.coalition_per_line,
                            &thread_coalition,
                        )
                        .with_context(|| format!("could not write body for {thread_coalition}"))
                        .unwrap();
                })
            })
            .collect();

        handles.into_iter().zip(coalitions).for_each(|(h, c)| {
            h.join()
                .unwrap_or_else(|_| println!("could not write data for {c}"))
        });
        Ok(())
    }
}
