use std::fs::File;
use std::io::Read;
// Importing the File struct from the standard library.

use plist::Value;
// Importing the Value enum from the plist crate.

use zip::ZipArchive;
// Importing the ZipArchive struct from the zip crate.

use crate::framework_info::LibraryInfo;
// Importing the LibraryInfo struct from the framework_info module.

use crate::utils::calculate_library_size;
// Importing the calculate_library_size function from the utils module.

use crate::framework_info::{FrameworkInfo, CollectedDataType, AccessedAPIType};
// Importing the FrameworkInfo, CollectedDataType, AccessedAPIType and PrivacyInfo structs from the framework_info module.

pub fn extract_privacy_info(archive: &mut ZipArchive<File>, framework_info: &mut FrameworkInfo) -> Result<(), String> {
    // Function to extract privacy information from PrivacyInfo.xcprivacy.
    let mut privacy_info_buffer = None;
    // Initializing a buffer to hold the privacy info data.
    for i in 0..archive.len() {
        // Iterating through each file in the archive.
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to access file in archive: {}", e))?;
        // Accessing the file by its index, returning an error if it fails.
        if file.name().ends_with("PrivacyInfo.xcprivacy") {
            // Checking if the file name ends with "PrivacyInfo.xcprivacy".
            let mut buffer = Vec::new();
            // Initializing a buffer to hold the file content.
            file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file content: {}", e))?;
            // Reading the file content into the buffer, returning an error if it fails.
            privacy_info_buffer = Some(buffer);
            // Storing the buffer in privacy_info_buffer.
            break;
            // Breaking the loop since the privacy info file is found.
        }
    }

    framework_info.privacy_info.present = false;
    // Setting privacy_info_present to false if the privacy info file is not found.

    if let Some(buffer) = privacy_info_buffer {
        // Checking if privacy_info_buffer is Some.
        let cursor = std::io::Cursor::new(buffer);
        // Creating a cursor from the buffer.
        let plist: Value = plist::from_reader(cursor).map_err(|e| format!("Failed to parse PrivacyInfo.xcprivacy: {}", e))?;
        // Parsing the plist data from the cursor, returning an error if it fails.
        if let Value::Dictionary(dict) = plist {
            framework_info.privacy_info.present = true;
            // Setting privacy_info_present to true.
            framework_info.privacy_info.tracking = dict.get("NSPrivacyTracking").and_then(|v| v.as_boolean());
            // Extracting the NSPrivacyTracking value.
            framework_info.privacy_info.tracking_domains = dict.get("NSPrivacyTrackingDomains").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect());
            // Extracting the NSPrivacyTrackingDomains value.
            
            // Extracting NSPrivacyCollectedDataTypes
            if let Some(collected_data_types) = dict.get("NSPrivacyCollectedDataTypes").and_then(|v| v.as_array()) {
                framework_info.privacy_info.collected_data_types = Some(collected_data_types.iter().filter_map(|item| {
                    if let Value::Dictionary(data_dict) = item {
                        Some(CollectedDataType {
                            collected_data_type: data_dict.get("NSPrivacyCollectedDataType")?.as_string()?.to_string(),
                            collected_data_type_linked: data_dict.get("NSPrivacyCollectedDataTypeLinked")?.as_boolean()?,
                            collected_data_type_tracking: data_dict.get("NSPrivacyCollectedDataTypeTracking")?.as_boolean()?,
                            collected_data_type_purposes: data_dict.get("NSPrivacyCollectedDataTypePurposes")?.as_array()?.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect(),
                        })
                    } else {
                        None
                    }
                }).collect::<Vec<CollectedDataType>>());
            }
            
            // Extracting NSPrivacyAccessedAPITypes
            if let Some(accessed_api_types) = dict.get("NSPrivacyAccessedAPITypes").and_then(|v| v.as_array()) {
                framework_info.privacy_info.accessed_api_types = Some(accessed_api_types.iter().filter_map(|item| {
                    if let Value::Dictionary(api_dict) = item {
                        Some(AccessedAPIType {
                            accessed_api_type: api_dict.get("NSPrivacyAccessedAPIType")?.as_string()?.to_string(),
                            accessed_api_type_reasons: api_dict.get("NSPrivacyAccessedAPITypeReasons")?.as_array()?.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect(),
                        })
                    } else {
                        None
                    }
                }).collect::<Vec<AccessedAPIType>>());
            }
        } else {
            return Err("PrivacyInfo.xcprivacy is not a dictionary".to_string());
        }
    }

    Ok(())
    // Returning Ok if the extraction is successful.
}

pub fn extract_framework_name(plist: &Value) -> Option<String> {
    // Function to extract the framework name from the plist.
    if let Value::Dictionary(dict) = plist {
        // Checking if the plist is a dictionary.
        if let Some(Value::String(name)) = dict.get("CFBundleName") {
            // Getting the CFBundleName value if it exists.
            return Some(name.clone());
            // Returning the framework name.
        }
    }
    None
    // Returning None if the framework name is not found.
}

pub fn extract_framework_version(plist: &Value) -> Option<String> {
    // Function to extract the framework version from the plist.
    if let Value::Dictionary(dict) = plist {
        // Checking if the plist is a dictionary.
        if let Some(Value::String(version)) = dict.get("CFBundleShortVersionString") {
            // Getting the CFBundleShortVersionString value if it exists.
            return Some(version.clone());
            // Returning the framework version.
        }
    }
    None
    // Returning None if the framework version is not found.
}

pub fn check_mergeable_metadata(plist: &Value) -> bool {
    // Function to check if mergeable metadata is present in the plist.
    if let Value::Dictionary(dict) = plist {
        // Checking if the plist is a dictionary.
        if let Some(available_libraries) = dict.get("AvailableLibraries").and_then(|v| v.as_array()) {
            // Getting the AvailableLibraries array if it exists.
            for library in available_libraries {
                // Iterating through each library in the array.
                if let Some(true) = library
                    .as_dictionary()
                    .and_then(|lib_dict| lib_dict.get("MergeableMetadata"))
                    .and_then(|v| v.as_boolean())
                {
                    // Checking if the MergeableMetadata key exists and is true.
                    return true;
                    // Returning true if mergeable metadata is found.
                }
            }
        }
    }
    false
    // Returning false if mergeable metadata is not found.
}

pub fn extract_built_for_distribution(plist: &Value) -> bool {
    // Function to extract the built-for-distribution flag from the plist.
    if let Value::Dictionary(dict) = plist {
        // Checking if the plist is a dictionary.
        if let Some(Value::Boolean(built_for_distribution)) = dict.get("DTSDKBuild") {
            // Getting the DTSDKBuild value if it exists.
            return *built_for_distribution;
            // Returning the built-for-distribution flag.
        }
    }
    false
    // Returning false if the built-for-distribution flag is not found.
}

pub fn extract_available_libraries(
    archive: &mut ZipArchive<File>,
    plist: &Value,
) -> Option<Vec<LibraryInfo>> {
    // Function to extract available libraries information from the plist.
    if let Value::Dictionary(dict) = plist {
        // Checking if the plist is a dictionary.
        if let Some(available_libraries) = dict.get("AvailableLibraries").and_then(|v| v.as_array()) {
            // Getting the AvailableLibraries array if it exists.
            let libraries: Vec<LibraryInfo> = available_libraries
                .iter()
                .filter_map(|library| {
                    // Iterating through each library in the array and collecting LibraryInfo structs.
                    if let Value::Dictionary(lib_dict) = library {
                        // Checking if the library is a dictionary.
                        let binary_path = lib_dict.get("BinaryPath")?.as_string()?.to_string();
                        // Getting the BinaryPath value if it exists.
                        let library_identifier =
                            lib_dict.get("LibraryIdentifier")?.as_string()?.to_string();
                        // Getting the LibraryIdentifier value if it exists.
                        let library_path = lib_dict.get("LibraryPath")?.as_string()?.to_string();
                        // Getting the LibraryPath value if it exists.
                        let mergeable_metadata = lib_dict
                            .get("MergeableMetadata")
                            .and_then(|v| v.as_boolean());
                        // Getting the MergeableMetadata value if it exists.
                        let supported_architectures = lib_dict
                            .get("SupportedArchitectures")?
                            .as_array()?
                            .iter()
                            .filter_map(|v| v.as_string().map(|s| s.to_string()))
                            .collect();
                        // Getting the SupportedArchitectures array if it exists and collecting the architectures.
                        let supported_platform =
                            lib_dict.get("SupportedPlatform")?.as_string()?.to_string();
                        // Getting the SupportedPlatform value if it exists.
                        let supported_platform_variant = lib_dict
                            .get("SupportedPlatformVariant")
                            .and_then(|v| v.as_string())
                            .map(|s| s.to_string());
                        // Getting the SupportedPlatformVariant value if it exists.
                        let minimum_os_version = lib_dict
                            .get("MinimumOSVersion")
                            .and_then(|v| v.as_string())
                            .map(|s| s.to_string());
                        // Getting the MinimumOSVersion value if it exists.
                        let size = calculate_library_size(archive, &binary_path).ok();
                        // Calculating the size of the library.

                        Some(LibraryInfo {
                            binary_path,
                            library_identifier,
                            library_path,
                            mergeable_metadata,
                            supported_architectures,
                            supported_platform,
                            supported_platform_variant,
                            minimum_os_version,
                            size,
                        })
                        // Returning the LibraryInfo struct.
                    } else {
                        None
                        // Returning None if the library is not a dictionary.
                    }
                })
                .collect();
            return Some(libraries);
            // Returning the collected libraries.
        }
    }
    None
    // Returning None if no available libraries are found.
}
