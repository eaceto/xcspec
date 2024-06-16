use std::fs::File;
// Importing the File struct from the standard library.

use std::io::{Cursor, Read};
// Importing the Cursor and Read traits from the standard library.

use std::path::Path;
// Importing the Path struct from the standard library.

use plist::Value;
// Importing the Value enum from the plist crate.

use zip::ZipArchive;
// Importing the ZipArchive struct from the zip crate.

use extractors::{
    check_mergeable_metadata, extract_available_libraries, extract_built_for_distribution,
    extract_framework_name, extract_framework_version, extract_privacy_info
};
// Importing various extraction functions from the extractors module.

use swift_details::extract_swift_details;
// Importing the extract_swift_details function from the swift_details module.

use crate::framework_info::{FrameworkInfo, PrivacyInfo};
// Importing the FrameworkInfo, CollectedDataType, and AccessedAPIType structs from the framework_info module.

pub mod extractors;
// Declaring the extractors module.

pub mod swift_details;
// Declaring the swift_details module.

pub fn collect_framework_info<P: AsRef<Path>>(zip_path: P) -> Result<FrameworkInfo, String> {
    // Function to collect information about a framework from a zip file.
    let file = File::open(&zip_path).map_err(|e| format!("Failed to open the zip file: {}", e))?;
    // Opening the zip file, returning an error if it fails.
    let mut archive =
        ZipArchive::new(file).map_err(|e| format!("Failed to read the zip file: {}", e))?;
    // Creating a ZipArchive instance from the opened file, returning an error if it fails.

    let privacy_info = PrivacyInfo {
        present: false,
        tracking: None,
        tracking_domains: None,
        collected_data_types: None,
        accessed_api_types: None,
    };

    let mut framework_info = FrameworkInfo {
        // Initializing the FrameworkInfo struct with default values.
        framework_name: None,
        framework_version: None,
        is_mergeable: false,
        swift_compiler_info: None,
        swift_compiler_version: None,
        swift_version: None,
        library_evolution_enabled: false,
        built_for_distribution: false,
        available_libraries: None,
        privacy_info: privacy_info
    };

    let mut plist_buffer = None;
    // Initializing a buffer to hold the plist data.
    for i in 0..archive.len() {
        // Iterating through each file in the archive.
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;
        // Accessing the file by its index, returning an error if it fails.
        if file.name().ends_with("Info.plist") {
            // Checking if the file name ends with "Info.plist".
            let mut buffer = Vec::new();
            // Initializing a buffer to hold the file content.
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("Failed to read file content: {}", e))?;
            // Reading the file content into the buffer, returning an error if it fails.
            plist_buffer = Some(buffer);
            // Storing the buffer in plist_buffer.
            break;
            // Breaking the loop since the plist file is found.
        }
    }

    if let Some(buffer) = plist_buffer {
        // Checking if plist_buffer is Some.
        let cursor = Cursor::new(buffer);
        // Creating a cursor from the buffer.
        let plist: Value =
            plist::from_reader(cursor).map_err(|e| format!("Failed to read Info.plist: {}", e))?;
        // Parsing the plist data from the cursor, returning an error if it fails.

        framework_info.framework_name = extract_framework_name(&plist);
        // Extracting the framework name.
        framework_info.framework_version = extract_framework_version(&plist);
        // Extracting the framework version.

        if check_mergeable_metadata(&plist) {
            framework_info.is_mergeable = true;
            // Setting is_mergeable to true if mergeable metadata is found.
        }

        extract_swift_details(&mut archive, &mut framework_info);
        // Extracting Swift-related details.

        framework_info.built_for_distribution = extract_built_for_distribution(&plist);
        // Extracting the built-for-distribution flag.

        framework_info.available_libraries = extract_available_libraries(&mut archive, &plist);
        // Extracting available libraries information.
    } else {
        return Err("Info.plist not found in the archive".to_string());
        // Returning an error if Info.plist is not found.
    }

    extract_privacy_info(&mut archive, &mut framework_info)?;
    // Extracting privacy information from PrivacyInfo.xcprivacy.

    Ok(framework_info)
    // Returning the collected framework information.
}
