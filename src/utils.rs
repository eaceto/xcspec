use std::fs::File;
use zip::ZipArchive;
use indexmap::IndexSet;

/// Ensures the elements in the vector are unique while preserving the order.
pub fn ensure_unique_ordered(vec: Vec<String>) -> Vec<String> {
    let set: IndexSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

/// Calculates the size of a library within a zip archive.
pub fn calculate_library_size(
    archive: &mut ZipArchive<File>,
    binary_path: &str,
) -> Result<String, String> {
    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;
        if file.name().ends_with(binary_path) {
            let size = file.size();
            return Ok(if size >= 1_000_000 {
                format!("{:.2} MB", size as f64 / 1_000_000.0)
            } else {
                format!("{:.2} KB", size as f64 / 1_000.0)
            });
        }
    }
    Err(format!("Binary file '{}' not found in archive.", binary_path))
}
