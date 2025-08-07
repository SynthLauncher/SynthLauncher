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

/// Similar to [`copy_dir_all`], but asynchronous
pub async fn async_copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> std::io::Result<()> {
    tokio::fs::create_dir_all(&dst).await?;
    let mut entries = tokio::fs::read_dir(src).await?;

    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;

        let src_path = entry.path();
        let dest_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            Box::pin(async_copy_dir_all(src_path, dest_path)).await?
        } else {
            tokio::fs::copy(src_path, dest_path).await?;
        }
    }
    Ok(())
}
