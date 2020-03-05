use std::fs;

/**
 * Read a source file
 * **filename does not include the extension**
 */
pub fn read_source_file(mut filename: String, work_dir: String) -> String {
    if !filename.ends_with(".eye") {
        filename += ".eye";
    }
    let source_file_path = format!("{}/{}", work_dir, filename);

    let fail_str = format!("Failed to read file: {}", source_file_path).to_string();

    fs::read_to_string(source_file_path).expect(&fail_str)
}
