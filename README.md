# xcspec

A tool to extract and format information of iOS, macOS, and Apple-like frameworks & libraries. It provides details about the framework, including its name, version, supported platforms, architectures, Swift version, and more.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Output Formats](#output-formats)
    - [JSON Output (default)](#json-output-default)
    - [YAML Output](#yaml-output)
  - [Output to a File](#output-to-a-file)
- [Example Output](#example-output)
  - [JSON](#json)
  - [YAML](#yaml)
- [Contributing](#contributing)
- [Author](#author)

## Features

- Extracts framework name and version
- Checks if the framework is mergeable
- Extracts Swift compiler and Swift version details
- Provides details about available libraries within the framework
- Provides information about Privacy Manifests (if `PrivacyInfo.xcprivacy` file exists)

## Installation

You can install xcspec using Homebrew:

```sh
brew tap eaceto/tools
brew install eaceto/tools/xcspec
```

## Usage

After installation, you can use the xcspec command to inspect a zipped framework file.

### Basic Usage

```sh
xcspec path_to_xcframework.zip
xcspec -f path_to_xcframework.zip
xcspec --file path_to_xcframework.zip
```

### Output formats

By default, the output is in JSON format. You can also export the result as YAML using the **-t** or **--output-format** flag.

#### JSON Output (default)

```sh
xcspec --file path_to_xcframework.zip
xcspec --file path_to_xcframework.zip -t json
xcspec --file path_to_xcframework.zip --output-format json
```

#### YAML Output

```sh
xcspec --file path_to_xcframework.zip -t yaml
xcspec --file path_to_xcframework.zip --output-format yaml
```

### Output to a File

Write the output to a file can be addressed directly from the tool by defining an output (**-o** / **--output**) flag.


```sh
xcspec --file path_to_xcframework.zip -t yaml -o info.yaml
xcspec --file path_to_xcframework.zip -o info.json
xcspec --file path_to_xcframework.zip -output info.json
```

### Example Output

#### JSON

```json
{
  "framework_name": "Multiplatform",
  "framework_version": "1.0",
  "is_mergeable": false,
  "swift_compiler_info": "Apple Swift version 5.10 (swiftlang-5.10.0.13 clang-1500.3.9.4)",
  "swift_compiler_version": "5.10.0.13",
  "swift_version": "5",
  "library_evolution_enabled": true,
  "built_for_distribution": false,
  "privacy_info": {
    "present": true,
    "tracking": true,
    "tracking_domains": [
      "ep1.facebook.com"
    ],
    "collected_data_types": [
      {
        "collected_data_type": "NSPrivacyCollectedDataTypeOtherDataTypes",
        "collected_data_type_linked": false,
        "collected_data_type_tracking": false,
        "collected_data_type_purposes": [
          "NSPrivacyCollectedDataTypePurposeAnalytics"
        ]
      },
      {
        "collected_data_type": "NSPrivacyCollectedDataTypeCrashData",
        "collected_data_type_linked": false,
        "collected_data_type_tracking": false,
        "collected_data_type_purposes": [
          "NSPrivacyCollectedDataTypePurposeAppFunctionality"
        ]
      },
      {
        "collected_data_type": "NSPrivacyCollectedDataTypeDeviceID",
        "collected_data_type_linked": true,
        "collected_data_type_tracking": true,
        "collected_data_type_purposes": [
          "NSPrivacyCollectedDataTypePurposeThirdPartyAdvertising",
          "NSPrivacyCollectedDataTypePurposeAppFunctionality",
          "NSPrivacyCollectedDataTypePurposeAnalytics"
        ]
      }
    ],
    "accessed_api_types": [
      {
        "accessed_api_type": "NSPrivacyAccessedAPICategoryUserDefaults",
        "accessed_api_type_reasons": [
          "CA92.1"
        ]
      }
    ]
  }
}
```

#### YAML

````yml
framework_name: Multiplatform
framework_version: '1.0'
is_mergeable: false
swift_compiler_info: Apple Swift version 5.10 (swiftlang-5.10.0.13 clang-1500.3.9.4)
swift_compiler_version: 5.10.0.13
swift_version: '5'
library_evolution_enabled: true
built_for_distribution: false
privacy_info:
  present: true
  tracking: true
  tracking_domains:
  - ep1.facebook.com
  collected_data_types:
  - collected_data_type: NSPrivacyCollectedDataTypeOtherDataTypes
    collected_data_type_linked: false
    collected_data_type_tracking: false
    collected_data_type_purposes:
    - NSPrivacyCollectedDataTypePurposeAnalytics
  - collected_data_type: NSPrivacyCollectedDataTypeCrashData
    collected_data_type_linked: false
    collected_data_type_tracking: false
    collected_data_type_purposes:
    - NSPrivacyCollectedDataTypePurposeAppFunctionality
  - collected_data_type: NSPrivacyCollectedDataTypeDeviceID
    collected_data_type_linked: true
    collected_data_type_tracking: true
    collected_data_type_purposes:
    - NSPrivacyCollectedDataTypePurposeThirdPartyAdvertising
    - NSPrivacyCollectedDataTypePurposeAppFunctionality
    - NSPrivacyCollectedDataTypePurposeAnalytics
  accessed_api_types:
  - accessed_api_type: NSPrivacyAccessedAPICategoryUserDefaults
    accessed_api_type_reasons:
    - CA92.1
````

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

## Author

Ezequiel (Kimi) Aceto

* [My blog](https://eaceto.dev)
* [Find my on LinkedIn](https://es.linkedin.com/in/ezequielaceto)