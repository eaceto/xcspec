use clap::{arg, command, Parser};
// Importing macros and traits from the clap crate.

use xcframework_processing::collect_framework_info;
// Importing the collect_framework_info function from the xcframework_processing module.

mod framework_info;
// Declaring the framework_info module.

mod xcframework_processing;
// Declaring the xcframework_processing module.

mod utils;
// Declaring the utils module.

use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A tool to extract and format information of iOS, macOS, and Apple-like framework & libraries",
    long_about = None,
    author = "Ezequiel (Kimi) Aceto <ezequiel.aceto@gmail.com>",
    after_help = "Copyright © 2024 Ezequiel (Kimi) Aceto"
)]
// Defining the Args struct with the Parser and Debug traits. This struct will handle command-line arguments.
struct Args {
    /// path of the framework to analyse
    #[arg(short, long)]
    file: Option<String>,
    // Optional argument for the framework file path.

    /// export format of the report
    #[arg(value_enum, long, short = 't', default_value_t = ExportFormat::JSON)]
    output_format: ExportFormat,
    // Argument for the export format, defaulting to JSON.

    /// positional argument for the framework's path to analyse
    #[arg(value_name = "FILE_PATH", required_unless_present = "file", conflicts_with = "file")]
    positional_file: Option<String>,
    // Positional argument for the framework file path, required unless the file argument is provided.

    /// output file path
    #[arg(short, long)]
    output: Option<String>,
    // Optional argument for the output file path.
}

#[derive(clap::ValueEnum, Clone, Debug)]
// Enum to specify the export format options.
enum ExportFormat {
    JSON,
    YAML,
}

fn main() {
    let args = Args::parse();
    // Parsing the command-line arguments into an instance of Args.

    let zip_path = args.file.or(args.positional_file).expect("File path is required");
    // Getting the file path from either the file argument or the positional argument.

    let output_format: ExportFormat = args.output_format;
    // Getting the export format from the arguments.

    match collect_framework_info(zip_path) {
        // Collecting framework information.
        Ok(info) => {
            // If successful, serialize the info to the desired format and print it or write it to a file.
            let output = match output_format {
                ExportFormat::YAML => {
                    serde_yaml::to_string(&info).expect("Failed to serialize to YAML")
                    // Serialize to YAML if specified.
                }
                ExportFormat::JSON => {
                    serde_json::to_string_pretty(&info).expect("Failed to serialize to JSON")
                    // Serialize to JSON if specified.
                }
            };

            if let Some(output_path) = args.output {
                // If the output argument is provided, write the output to the specified file.
                let mut file = File::create(output_path).expect("Failed to create output file");
                file.write_all(output.as_bytes()).expect("Failed to write to output file");
            } else {
                // Otherwise, print the output to the console.
                println!("{}", output);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
        // Print the error if the information collection fails.
    }
}
