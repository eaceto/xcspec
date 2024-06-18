use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use plist::Value;
use zip::ZipArchive;
use crate::framework_info::FrameworkInfo;

pub mod extractors;
pub mod swift_details;

pub fn collect_framework_info<P: AsRef<Path>>(zip_path: P) -> Result<FrameworkInfo, String> {
    let file = File::open(&zip_path).map_err(|e| format!("Failed to open the zip file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read the zip file: {}", e))?;

    let mut framework_info = FrameworkInfo {
        framework_name: None,
        is_mergeable: false,
        swift_compiler_info: None,
        swift_compiler_version: None,
        swift_version: None,
        library_evolution_enabled: false,
        built_for_distribution: false,
        available_libraries: None,
    };

    let mut plist_buffer = None;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to access file in archive: {}", e))?;
        if file.name().ends_with("Info.plist") {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file content: {}", e))?;
            plist_buffer = Some(buffer);
            break;
        }
    }

    if let Some(buffer) = plist_buffer {
        let cursor = Cursor::new(buffer);
        let plist: Value = plist::from_reader(cursor).map_err(|e| format!("Failed to read Info.plist: {}", e))?;

        framework_info.framework_name = extractors::extract_framework_name(&plist);
        framework_info.is_mergeable = extractors::check_mergeable_metadata(&plist);
        swift_details::extract_swift_details(&mut archive, &mut framework_info);
        framework_info.built_for_distribution = extractors::extract_built_for_distribution(&mut archive, &plist);
        framework_info.available_libraries = extractors::extract_available_libraries(&mut archive, &plist);
    } else {
        return Err("Info.plist not found in the archive".to_string());
    }

    Ok(framework_info)
}
