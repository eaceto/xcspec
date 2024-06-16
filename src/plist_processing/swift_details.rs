use std::fs::File;
// Importing the File struct from the standard library.

use std::io::{BufRead, BufReader};
// Importing the BufRead and BufReader traits from the standard library.

use zip::ZipArchive;
// Importing the ZipArchive struct from the zip crate.

use crate::framework_info::FrameworkInfo;
// Importing the FrameworkInfo struct from the framework_info module.

pub fn extract_swift_details(archive: &mut ZipArchive<File>, framework_info: &mut FrameworkInfo) {
    // Function to extract Swift-related details from .swiftinterface files.
    for i in 0..archive.len() {
        // Iterating through each file in the archive.
        if let Ok(mut file) = archive.by_index(i) {
            // Accessing the file by its index, continuing if it fails.
            if file.name().ends_with(".swiftinterface") {
                // Checking if the file name ends with ".swiftinterface".
                let reader = BufReader::new(&mut file);
                // Creating a BufReader for the file.
                for line in reader.lines() {
                    // Iterating through each line in the file.
                    if let Ok(line) = line {
                        // Checking if reading the line is successful.
                        if line.starts_with("// swift-compiler-version: ") {
                            let compiler_info = line
                                .trim_start_matches("// swift-compiler-version: ")
                                .to_string();
                            // Extracting the swift-compiler-version.
                            framework_info.swift_compiler_info = Some(compiler_info.clone());
                            // Storing the swift-compiler-version.

                            if let Some(swiftlang_version) = compiler_info
                                .split_whitespace()
                                .find(|&part| part.contains("swiftlang-"))
                            {
                                let version = swiftlang_version
                                    .split('-')
                                    .nth(1)
                                    .unwrap_or("")
                                    .to_string();
                                // Extracting the swiftlang version.
                                framework_info.swift_compiler_version = Some(version);
                                // Storing the swiftlang version.
                            }
                        } else if line.starts_with("// swift-module-flags: ") {
                            let flags = line
                                .trim_start_matches("// swift-module-flags: ")
                                .split_whitespace()
                                .collect::<Vec<&str>>();
                            // Extracting the swift-module-flags.
                            for (i, flag) in flags.iter().enumerate() {
                                if *flag == "-module-name" && i + 1 < flags.len() {
                                    framework_info.framework_name = Some(flags[i + 1].to_string());
                                    // Extracting and storing the module name.
                                } else if *flag == "-swift-version" && i + 1 < flags.len() {
                                    framework_info.swift_version = Some(flags[i + 1].to_string());
                                    // Extracting and storing the swift version.
                                } else if *flag == "-enable-library-evolution" {
                                    framework_info.library_evolution_enabled = true;
                                    // Setting library_evolution_enabled to true if the flag is present.
                                }
                            }
                        }
                    }
                }
                break;
                // Breaking the loop since the swiftinterface file is processed.
            }
        }
    }
}
