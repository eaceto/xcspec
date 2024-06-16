use std::fs::File;
// Importing the File struct from the standard library.

use zip::ZipArchive;
// Importing the ZipArchive struct from the zip crate.

pub fn calculate_library_size(
    archive: &mut ZipArchive<File>,
    binary_path: &str,
) -> Result<String, String> {
    // Function to calculate the size of a library within a zip archive.
    for i in 0..archive.len() {
        // Iterating through each file in the archive.
        let file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;
        // Accessing the file by its index, returning an error if it fails.
        if file.name().ends_with(binary_path) {
            // Checking if the file name ends with the specified binary path.
            let size = file.size();
            // Getting the size of the file.
            return Ok(if size >= 1_000_000 {
                format!("{:.2} MB", size as f64 / 1_000_000.0)
                // Formatting size in MB if it's 1,000,000 bytes or more.
            } else {
                format!("{:.2} KB", size as f64 / 1_000.0)
                // Formatting size in KB if it's less than 1,000,000 bytes.
            });
        }
    }
    Err("Binary file not found in archive.".to_string())
    // Returning an error if the binary file is not found.
}
