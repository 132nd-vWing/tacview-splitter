pub mod lib {
    use std::io::Write;
    use std::fs;

    use zip;

    const ERR_CANNOT_WRITE_DATA: &str = "Could not write data";
    const ERR_CANNOT_OPEN_OUTPUT: &str = "Could not open output file";
    const ERR_CANNOT_BEGIN_FILE: &str = "Could not begin file in zip archive";

    pub struct IDs<'a>  {
        pub blue: Vec<&'a str>,
        pub red: Vec<&'a str>,
        pub violet: Vec<&'a str>,
        pub unknown: Vec<&'a str>,
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
                write!(self.blue, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
                write!(self.red, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
                write!(self.violet, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.blue {
                write!(self.blue, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.red {
                write!(self.red, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
            }
            for line in &bodies_by_coalition.violet {
                write!(self.violet, "{}\n", line).expect(ERR_CANNOT_WRITE_DATA);
            }
        }
    }

    impl Descriptors<fs::File> {
        pub fn new(filenames: OutputFilenames) -> Descriptors<fs::File> {
            let blue = fs::File::create(&filenames.txt.blue).expect(ERR_CANNOT_OPEN_OUTPUT);
            let red = fs::File::create(&filenames.txt.red).expect(ERR_CANNOT_OPEN_OUTPUT);
            let violet = fs::File::create(&filenames.txt.violet).expect(ERR_CANNOT_OPEN_OUTPUT);
            let descriptors = Descriptors { blue, red, violet };
            return descriptors
        }
    }

    impl Descriptors<zip::ZipWriter<fs::File>> {
        pub fn new(filenames: OutputFilenames) -> Descriptors<zip::ZipWriter<fs::File>> {
            let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

            let file = fs::File::create(&filenames.zip.blue).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut blue = zip::ZipWriter::new(file);
            blue.start_file(&filenames.txt.blue, options).expect(ERR_CANNOT_BEGIN_FILE);

            let file = fs::File::create(&filenames.zip.red).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut red = zip::ZipWriter::new(file);
            red.start_file(&filenames.txt.red, options).expect(ERR_CANNOT_BEGIN_FILE);

            let file = fs::File::create(&filenames.zip.violet).expect(ERR_CANNOT_OPEN_OUTPUT);
            let mut violet = zip::ZipWriter::new(file);
            violet.start_file(&filenames.txt.violet, options).expect(ERR_CANNOT_BEGIN_FILE);

            let descriptors = Descriptors{blue, red, violet};
            descriptors
        }
    }

    pub fn sanity_check_output_filenames(input_filename: &String, output_filenames: &FilenamesVariant) {
        if input_filename == &output_filenames.blue ||
            input_filename == &output_filenames.red ||
            input_filename == &output_filenames.violet {
            panic!("Output filenames were the same as input filenames")
        }
    }

    pub fn get_output_filenames_individual(input_filename: &String, old_extension: &str, new_extension: &str, coalition: &str) -> String {
        let mut output_extension = coalition.to_owned();
        output_extension.push_str(new_extension);
        let output_filename = input_filename.replace(old_extension, &output_extension);
        output_filename
    }



}
}
