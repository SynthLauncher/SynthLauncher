use std::path::Path;

/// Some helper function to recursively copy a directory
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        let src_path = entry.path();
        let dest_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(src_path, dest_path)?
        } else {
            std::fs::copy(src_path, dest_path)?;
        }
    }
    Ok(())
}
