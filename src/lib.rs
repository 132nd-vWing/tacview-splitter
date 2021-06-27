pub mod lib {
    use std::io::Write;
    use std::fs;
    use zip;

    const ERR_CANNOT_WRITE_DATA: &str = "Could not write data";

    pub struct IDs<'a>  {
        pub blue: Vec<&'a str>,
        pub red: Vec<&'a str>,
        pub violet: Vec<&'a str>,
        pub unknown: Vec<&'a str>,
    }

    pub struct DescriptorsZip {
        blue: zip::ZipWriter<fs::File>,
        red: zip::ZipWriter<fs::File>,
        violet: zip::ZipWriter<fs::File>,
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

    pub trait WriteData {
        fn write(&mut self, header: Vec<String>, bodies_by_coalition: BodiesByCoalition);
    }

    impl WriteData for DescriptorsTxt {
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

    impl WriteData for DescriptorsZip {
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

}

