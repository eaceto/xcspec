use std::fs::File;
use std::io::{Cursor, Read};
use plist::Value;
use zip::ZipArchive;
use crate::framework_info::{LibraryInfo, PrivacyInfo};
use crate::utils::{calculate_library_size, ensure_unique_ordered};
use crate::framework_info::{AccessedAPIType, CollectedDataType};

pub fn extract_privacy_info(
    archive: &mut ZipArchive<File>,
    library_identifier: &str,
    library_path: &str
) -> Result<PrivacyInfo, String> {
    let mut privacy_info_buffer = None;
    let modules_path = format!("{}/{}", library_identifier, library_path);

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;
        if file.name().contains(&modules_path) && file.name().ends_with("PrivacyInfo.xcprivacy") {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file content: {}", e))?;
            privacy_info_buffer = Some(buffer);
            break;
        }
    }

    let mut privacy_info = PrivacyInfo {
        present: false,
        tracking: None,
        tracking_domains: None,
        collected_data_types: None,
        accessed_api_types: None,
    };

    if let Some(buffer) = privacy_info_buffer {
        let cursor = std::io::Cursor::new(buffer);
        let plist: Value = plist::from_reader(cursor)
            .map_err(|e| format!("Failed to parse PrivacyInfo.xcprivacy: {}", e))?;
        if let Value::Dictionary(dict) = plist {
            privacy_info.present = true;
            privacy_info.tracking = dict.get("NSPrivacyTracking").and_then(|v| v.as_boolean());
            privacy_info.tracking_domains = dict.get("NSPrivacyTrackingDomains").and_then(|v| v.as_array()).map(|arr| {
                arr.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect()
            });

            if let Some(collected_data_types) = dict.get("NSPrivacyCollectedDataTypes").and_then(|v| v.as_array()) {
                privacy_info.collected_data_types = Some(
                    collected_data_types.iter().filter_map(|item| {
                        if let Value::Dictionary(data_dict) = item {
                            Some(CollectedDataType {
                                data_type: data_dict.get("NSPrivacyCollectedDataType")?.as_string()?.to_string(),
                                linked_to_user: data_dict.get("NSPrivacyCollectedDataTypeLinked")?.as_boolean()?,
                                tracking: data_dict.get("NSPrivacyCollectedDataTypeTracking")?.as_boolean()?,
                                purposes: data_dict.get("NSPrivacyCollectedDataTypePurposes")?.as_array()?.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect(),
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<CollectedDataType>>(),
                );
            }

            if let Some(accessed_api_types) = dict.get("NSPrivacyAccessedAPITypes").and_then(|v| v.as_array()) {
                privacy_info.accessed_api_types = Some(
                    accessed_api_types.iter().filter_map(|item| {
                        if let Value::Dictionary(api_dict) = item {
                            Some(AccessedAPIType {
                                api: api_dict.get("NSPrivacyAccessedAPIType")?.as_string()?.to_string(),
                                reasons: api_dict.get("NSPrivacyAccessedAPITypeReasons")?.as_array()?.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect(),
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<AccessedAPIType>>(),
                );
            }
        } else {
            return Err("PrivacyInfo.xcprivacy is not a dictionary".to_string());
        }
    }

    Ok(privacy_info)
}

pub fn extract_framework_name(plist: &Value) -> Option<String> {
    if let Value::Dictionary(dict) = plist {
        if let Some(Value::String(name)) = dict.get("CFBundleName") {
            return Some(name.clone());
        }
    }
    None
}

pub fn extract_framework_version(
    archive: &mut ZipArchive<File>,
    library_identifier: &str,
    library_path: &str
) -> Option<String> {
    let modules_path = format!("{}/{}", library_identifier, library_path);

    let mut plist_buffer = None;
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e)).ok()?;
        if file.name().contains(&modules_path) && file.name().ends_with("Info.plist") {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file content: {}", e)).ok()?;
            plist_buffer = Some(buffer);
            break;
        }
    }

    if let Some(buffer) = plist_buffer {
        let cursor = Cursor::new(buffer);
        let plist: Value = plist::from_reader(cursor).map_err(|e| format!("Failed to read Info.plist: {}", e)).ok()?;

        if let Value::Dictionary(dict) = plist {
            if let Some(Value::String(version)) = dict.get("CFBundleShortVersionString") {
                return Some(version.clone());
            }
        }
    }
    None
}

pub fn check_mergeable_metadata(plist: &Value) -> bool {
    if let Value::Dictionary(dict) = plist {
        if let Some(available_libraries) = dict.get("AvailableLibraries").and_then(|v| v.as_array()) {
            for library in available_libraries {
                if let Some(true) = library.as_dictionary()
                    .and_then(|lib_dict| lib_dict.get("MergeableMetadata"))
                    .and_then(|v| v.as_boolean()) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn extract_built_for_distribution(archive: &mut ZipArchive<File>, plist: &Value) -> bool {
    if let Value::Dictionary(dict) = plist {
        if let Some(Value::Boolean(built_for_distribution)) = dict.get("DTSDKBuild") {
            return *built_for_distribution;
        }
    }

    // Check for .swiftinterface files in the Modules directory
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            if file.name().ends_with(".swiftinterface") {
                return true;
            }
        }
    }

    false
}

pub fn extract_available_libraries(
    archive: &mut ZipArchive<File>,
    plist: &Value,
) -> Option<Vec<LibraryInfo>> {
    if let Value::Dictionary(dict) = plist {
        if let Some(available_libraries) = dict.get("AvailableLibraries").and_then(|v| v.as_array()) {
            let libraries: Vec<LibraryInfo> = available_libraries.iter().filter_map(|library| {
                if let Value::Dictionary(lib_dict) = library {
                    let binary_path = lib_dict.get("BinaryPath")?.as_string()?.to_string();
                    let library_identifier = lib_dict.get("LibraryIdentifier")?.as_string()?.to_string();
                    let library_path = lib_dict.get("LibraryPath")?.as_string()?.to_string();
                    let mergeable_metadata = lib_dict.get("MergeableMetadata").and_then(|v| v.as_boolean());
                    let supported_architectures = lib_dict.get("SupportedArchitectures")?.as_array()?.iter().filter_map(|v| v.as_string().map(|s| s.to_string())).collect();
                    let supported_platform = lib_dict.get("SupportedPlatform")?.as_string()?.to_string();
                    let supported_platform_variant = lib_dict.get("SupportedPlatformVariant").and_then(|v| v.as_string()).map(|s| s.to_string());
                    let minimum_os_version = lib_dict.get("MinimumOSVersion").and_then(|v| v.as_string()).map(|s| s.to_string());
                    let size = calculate_library_size(archive, &binary_path).ok();
                    let marketing_version = extract_framework_version(archive, &library_identifier, &library_path);
                    let dependencies = extract_dependencies(archive, &library_identifier, &library_path);
                    let privacy_info = extract_privacy_info(archive, &library_identifier, &library_path).ok();

                    Some(LibraryInfo {
                        binary_path,
                        library_identifier,
                        library_path,
                        marketing_version,
                        mergeable_metadata,
                        supported_architectures,
                        supported_platform,
                        supported_platform_variant,
                        minimum_os_version,
                        size,
                        dependencies,
                        privacy_info,
                    })
                } else {
                    None
                }
            }).collect();
            return Some(libraries);
        }
    }
    None
}

fn extract_dependencies(archive: &mut ZipArchive<File>, library_identifier: &str, library_path: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    let modules_path = format!("{}/{}/Modules", library_identifier, library_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.name().contains(&modules_path) && file.name().ends_with(".swiftinterface") {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            for line in contents.lines() {
                if line.starts_with("import ") {
                    if let Some(dep) = line.split_whitespace().nth(1) {
                        // if !dep.starts_with('_') {
                        dependencies.push(dep.to_string());
                        // }
                    }
                }
            }
        }
    }

    ensure_unique_ordered(dependencies)
}
