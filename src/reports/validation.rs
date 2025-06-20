use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn validate_output_directory(output_dir: &PathBuf) -> Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
    }

    let mut test_file_path = output_dir.clone();
    test_file_path.push(".parsentry_write_test");

    match fs::File::create(&test_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(b"test") {
                let _ = fs::remove_file(&test_file_path);
                return Err(anyhow::anyhow!("No write permission: {}", e));
            }
            drop(file);
            fs::remove_file(&test_file_path)
                .map_err(|e| anyhow::anyhow!("Failed to delete test file: {}", e))?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("No file creation permission: {}", e));
        }
    }

    Ok(())
}