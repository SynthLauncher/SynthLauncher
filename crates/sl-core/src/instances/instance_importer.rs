use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom},
    path::Path,
};

use sl_utils::{errors::InstanceImportErr, zip::ZipExtractor};
use tempfile::TempDir;

use crate::instances::{instance_metadata::InstanceMetadata, InstanceManager, INSTANCE_FILE_NAME};

/// Imports an Instance exported in a Zip format from a file at `file_path`
pub(crate) async fn import_instance_from_path(
    man: &mut InstanceManager<'_>,
    file_path: &Path,
) -> Result<(), InstanceImportErr> {
    import_instance(man, BufReader::new(File::open(file_path)?)).await
}

// TODO: make more async
/// Imports an Instance exported in a Zip format, from a reader
pub(crate) async fn import_instance<R: Read + Seek>(
    man: &mut InstanceManager<'_>,
    reader: R,
) -> Result<(), InstanceImportErr> {
    // ==================================
    // Extract & Cache the data to Import
    let cache_dir = TempDir::new()?;
    let cache_dir_path = cache_dir.path();
    assert!(cache_dir_path.exists());

    let extractor = ZipExtractor::new(reader);
    extractor.extract(cache_dir_path)?;
    let cached_instance_file_path = cache_dir_path.join(INSTANCE_FILE_NAME);

    if !cached_instance_file_path.exists() {
        return Err(InstanceImportErr::NotAnInstance);
    }

    let mut cached_instance_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(cached_instance_file_path)?;

    let cached_instance_file_reader = BufReader::new(&mut cached_instance_file);
    // =======================
    // validate and modify the instance metadata until we get a valid metadata (name)
    let mut instance_metadata: InstanceMetadata =
        serde_json::from_reader(cached_instance_file_reader)
            .map_err(|_| InstanceImportErr::Corrupted)?;

    let orig_name = &instance_metadata.name;

    let mut name = instance_metadata.name.clone();
    let mut instance_import_path = man.instance_file(&name);
    let mut n = 1;

    while std::fs::exists(&instance_import_path).is_ok_and(|e| e) {
        name = format!("{orig_name} ({n})");
        instance_import_path = man.instance_file(&name);
        n += 1;
    }

    instance_metadata.name = name;
    // ======================
    // write the new metadata

    cached_instance_file.set_len(0)?;
    cached_instance_file.seek(SeekFrom::Start(0))?;

    let cached_instance_file_writer = BufWriter::new(cached_instance_file);
    serde_json::to_writer_pretty(cached_instance_file_writer, &instance_metadata)?;

    // =======================
    // Copy the cached data to the destination
    sl_utils::fs::async_copy_dir_all(cache_dir_path, instance_import_path).await?;
    Ok(())
}
