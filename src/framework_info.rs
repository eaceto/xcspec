use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrameworkInfo {
    pub framework_name: Option<String>,
    pub is_mergeable: bool,
    pub swift_compiler_info: Option<String>,
    pub swift_compiler_version: Option<String>,
    pub swift_version: Option<String>,
    pub library_evolution_enabled: bool,
    pub built_for_distribution: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_libraries: Option<Vec<LibraryInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryInfo {
    pub binary_path: String,
    pub library_identifier: String,
    pub library_path: String,
    pub marketing_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable_metadata: Option<bool>,
    pub supported_architectures: Vec<String>,
    pub supported_platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_platform_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_info: Option<PrivacyInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectedDataType {
    pub data_type: String,
    pub linked_to_user: bool,
    pub tracking: bool,
    pub purposes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessedAPIType {
    pub api: String,
    pub reasons: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivacyInfo {
    pub present: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_data_types: Option<Vec<CollectedDataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed_api_types: Option<Vec<AccessedAPIType>>,
}
