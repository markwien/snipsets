use dicom_parser::{DicomParser, DatasetParser};
use std::fs;

fn main() {
    // Replace the directory path below with the path to your directory of interest
    let directory_path = "/path/to/directory";

    // Read the contents of the directory
    let paths = fs::read_dir(directory_path).unwrap();

    // Iterate over the files in the directory and check if they are valid DICOM files
    for path in paths {
        if let Ok(entry) = path {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_path = entry.path();
                    if let Some(extension) = file_path.extension() {
                        if extension == "dcm" || extension == "DCM" {
                            match fs::read(&file_path) {
                                Ok(data) => {
                                    let parser = DicomParser::new();
                                    let dataset_parser = DatasetParser::new();
                                    match parser.parse(&data) {
                                        Ok(_header) => {
                                            match dataset_parser.parse(&data) {
                                                Ok(_dataset) => {
                                                    println!(
                                                        "File {} is a valid DICOM file.",
                                                        file_path.display()
                                                    );
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error parsing dataset in file {}: {}",
                                                        file_path.display(),
                                                        e
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error parsing header in file {}: {}",
                                                file_path.display(),
                                                e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Error reading file {}: {}",
                                        file_path.display(),
                                        e
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
