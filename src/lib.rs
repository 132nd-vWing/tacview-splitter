pub mod lib {
    use std::collections::HashSet;
    use std::fs;
    use std::io::Write;

    use zip;

    const ERR_CANNOT_WRITE_DATA: &str = "Could not write data";
    const ERR_CANNOT_OPEN_OUTPUT: &str = "Could not open output file";
    const ERR_CANNOT_BEGIN_FILE: &str = "Could not begin file in zip archive";

    pub struct IDs<'a> {
        pub blue: HashSet<&'a str>,
        pub red: HashSet<&'a str>,
        pub violet: HashSet<&'a str>,
        pub unknown: HashSet<&'a str>,
    }

    pub struct Descriptors<T: Write> {
        blue: T,
        red: T,
        violet: T,
    }

    pub struct DescriptorsTxt {
        pub blue: fs::File,
        pub red: fs::File,
        pub violet: fs::File,
    }

    pub struct OutputFilenames {
        pub txt: FilenamesVariant,
        pub zip: FilenamesVariant,
    }

    pub struct FilenamesVariant {
        pub blue: String,
        pub red: String,
        pub violet: String,
    }

    pub struct BodiesByCoalition<'a> {
        pub blue: Vec<&'a str>,
        pub red: Vec<&'a str>,
        pub violet: Vec<&'a str>,
    }

    pub trait Handling {
        fn write(&mut self, header: Vec<String>, bodies_by_coalition: BodiesByCoalition);
    }

    impl<T: Write> Handling for Descriptors<T> {
        fn write(&mut self, header: Vec<String>, bodies_by_coalition: BodiesByCoalition) {
            for line in &header {
                writeln!(self.blue, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
                writeln!(self.red, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
                writeln!(self.violet, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.blue {
                writeln!(self.blue, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.red {
                writeln!(self.red, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.violet {
                writeln!(self.violet, "{}", line).expect(ERR_CANNOT_WRITE_DATA);
            }
        }
    }

    impl Descriptors<fs::File> {
        pub fn new(filenames: OutputFilenames) -> Descriptors<fs::File> {
            let blue = fs::File::create(&filenames.txt.blue).expect(ERR_CANNOT_OPEN_OUTPUT);
            let red = fs::File::create(&filenames.txt.red).expect(ERR_CANNOT_OPEN_OUTPUT);
            let violet = fs::File::create(&filenames.txt.violet).expect(ERR_CANNOT_OPEN_OUTPUT);
            Descriptors { blue, red, violet }
        }
    }

    impl Descriptors<zip::ZipWriter<fs::File>> {
        pub fn new(filenames: OutputFilenames) -> Descriptors<zip::ZipWriter<fs::File>> {
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            let file = fs::File::create(&filenames.zip.blue).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut blue = zip::ZipWriter::new(file);
            blue.start_file(&filenames.txt.blue, options)
                .expect(ERR_CANNOT_BEGIN_FILE);

            let file = fs::File::create(&filenames.zip.red).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut red = zip::ZipWriter::new(file);
            red.start_file(&filenames.txt.red, options)
                .expect(ERR_CANNOT_BEGIN_FILE);

            let file = fs::File::create(&filenames.zip.violet).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut violet = zip::ZipWriter::new(file);
            violet
                .start_file(&filenames.txt.violet, options)
                .expect(ERR_CANNOT_BEGIN_FILE);

            Descriptors { blue, red, violet }
        }
    }

    pub fn sanity_check_output_filenames(
        input_filename: &String,
        output_filenames: &FilenamesVariant,
    ) {
        if input_filename == &output_filenames.blue
            || input_filename == &output_filenames.red
            || input_filename == &output_filenames.violet
        {
            panic!("Output filenames were the same as input filenames")
        }
    }

    pub fn get_output_filenames_individual(
        input_filename: &str,
        old_extension: &str,
        new_extension: &str,
        coalition: &str,
    ) -> String {
        let mut output_extension = coalition.to_owned();
        output_extension.push_str(new_extension);
        input_filename.replace(old_extension, &output_extension)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::*;
    #[test]
    fn test_get_output_filenames_individual() {
        let input_filename = "something.txt.acmi".to_string();
        let extension = ".txt.acmi";
        let coalition = "_blue";
        let result =
            get_output_filenames_individual(&input_filename, extension, extension, coalition);
        let correct_result = "something_blue.txt.acmi";
        assert_eq!(result, correct_result);

        let extension_wrong = ".tXT.acmi";
        let result =
            get_output_filenames_individual(&input_filename, extension_wrong, extension, coalition);
        assert_ne!(result, correct_result);
    }
}
