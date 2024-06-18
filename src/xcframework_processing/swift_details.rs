use std::fs::File;
use std::io::{BufRead, BufReader};
use zip::ZipArchive;
use crate::framework_info::FrameworkInfo;

pub fn extract_swift_details(archive: &mut ZipArchive<File>, framework_info: &mut FrameworkInfo) {
    for i in 0..archive.len() {
        if let Ok(mut file) = archive.by_index(i) {
            if file.name().ends_with(".swiftinterface") {
                let reader = BufReader::new(&mut file);
                for line in reader.lines().filter_map(Result::ok) {
                    if line.starts_with("// swift-compiler-version: ") {
                        let compiler_info = line.trim_start_matches("// swift-compiler-version: ").to_string();
                        framework_info.swift_compiler_info = Some(compiler_info.clone());

                        if let Some(swiftlang_version) = compiler_info.split_whitespace().find(|&part| part.contains("swiftlang-")) {
                            let version = swiftlang_version.split('-').nth(1).unwrap_or("").to_string();
                            framework_info.swift_compiler_version = Some(version);
                        }
                    } else if line.starts_with("// swift-module-flags: ") {
                        let flags = line.trim_start_matches("// swift-module-flags: ").split_whitespace().collect::<Vec<&str>>();
                        for (i, flag) in flags.iter().enumerate() {
                            match *flag {
                                "-module-name" if i + 1 < flags.len() => framework_info.framework_name = Some(flags[i + 1].to_string()),
                                "-swift-version" if i + 1 < flags.len() => framework_info.swift_version = Some(flags[i + 1].to_string()),
                                "-enable-library-evolution" => framework_info.library_evolution_enabled = true,
                                _ => {}
                            }
                        }
                    }
                }
                break;
            }
        }
    }
}
