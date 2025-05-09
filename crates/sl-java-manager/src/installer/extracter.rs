use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use sl_utils::utils::errors::{BackendError, ZipExtractionError};

pub fn extract_package(package_path: &Path, java: &Path) -> Result<(), BackendError> {
    let file = File::open(package_path)?;
    let extension = package_path.extension().and_then(|s| s.to_str()).unwrap();

    match extension {
        "gz" => {
            let tar_gz = BufReader::new(file);
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);

            for entry in archive.entries()? {
                let mut entry = entry?;
                let path = entry.path()?.into_owned();

                if let Some(stripped) = path.iter().skip(1).collect::<PathBuf>().to_str() {
                    let dest_path = java.join(stripped);

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    entry.unpack(dest_path)?;
                }
            }
        }
        "zip" => {
            let mut archive = zip::ZipArchive::new(file)?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let mut name = file.name().to_string();

                if let Some(start) = name.find('/') {
                    name = name[start + 1..].to_string();
                }

                let dest_path = java.join(&name);

                if file.is_dir() {
                    fs::create_dir_all(&dest_path)?;
                } else {
                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    let mut outfile = File::create(&dest_path)?;
                    io::copy(&mut file, &mut outfile)?;
                }
            }
        }
        ex => {
            return Err(BackendError::ZipExtractionError(
                ZipExtractionError::UnsupportedFileExt(ex.to_string()),
            ))
        }
    }

    Ok(())
}
