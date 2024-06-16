use serde::{Deserialize, Serialize};
// Importing the Serialize and Deserialize traits from the serde crate.

#[derive(Serialize, Deserialize, Debug)]
// Struct to store information about a framework.
pub struct FrameworkInfo {
    pub framework_name: Option<String>,
    // Optional framework name.
    pub framework_version: Option<String>,
    // Optional framework version.
    pub is_mergeable: bool,
    // Boolean indicating if the framework is mergeable.
    pub swift_compiler_info: Option<String>,
    // Optional Swift compiler info.
    pub swift_compiler_version: Option<String>,
    // Optional Swift compiler version.
    pub swift_version: Option<String>,
    // Optional Swift version.
    pub library_evolution_enabled: bool,
    // Boolean indicating if library evolution is enabled.
    pub built_for_distribution: bool,
    // Boolean indicating if the framework is built for distribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_libraries: Option<Vec<LibraryInfo>>,
    // Optional list of available libraries, omitted from serialization if None.
    pub privacy_info: PrivacyInfo,
    // Privacy Info
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// Struct to store information about a library.
pub struct LibraryInfo {
    pub binary_path: String,
    // Path to the binary file.
    pub library_identifier: String,
    // Library identifier.
    pub library_path: String,
    // Path to the library file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable_metadata: Option<bool>,
    // Optional boolean indicating if mergeable metadata is present, omitted from serialization if None.
    pub supported_architectures: Vec<String>,
    // List of supported architectures.
    pub supported_platform: String,
    // Supported platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_platform_variant: Option<String>,
    // Optional supported platform variant, omitted from serialization if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_os_version: Option<String>,
    // Optional minimum OS version, omitted from serialization if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    // Optional size of the library, omitted from serialization if None.
}

#[derive(Serialize, Deserialize, Debug)]
// Struct to store information about a collected data type.
pub struct CollectedDataType {
    pub collected_data_type: String,
    pub collected_data_type_linked: bool,
    pub collected_data_type_tracking: bool,
    pub collected_data_type_purposes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
// Struct to store information about an accessed API type.
pub struct AccessedAPIType {
    pub accessed_api_type: String,
    pub accessed_api_type_reasons: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
// Struct to store information about a library.
pub struct PrivacyInfo {
    pub present: bool,
    // Boolean indicating if the PrivacyInfo.xcprivacy file is present.
    pub tracking: Option<bool>,
    // Optional boolean indicating if NSPrivacyTracking is enabled.
    pub tracking_domains: Option<Vec<String>>,
    // Optional list of domains in NSPrivacyTrackingDomains.
    pub collected_data_types: Option<Vec<CollectedDataType>>,
    // Optional list of collected data types.
    pub accessed_api_types: Option<Vec<AccessedAPIType>>,
    // Optional list of accessed API types.
}