use std::{fs, io::Cursor, path::Path};

use zip::{result::ZipError, ZipArchive};

pub struct ZipExtractor<'a> {
    bytes: &'a [u8],
    exclude: Option<&'a [&'a Path]>,
}

impl<'a> ZipExtractor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            exclude: None,
        }
    }

    pub fn exclude(mut self, exclude: &'a [&'a Path]) -> Self {
        self.exclude = Some(exclude);
        self
    }

    pub fn extract(self, output: &Path) -> Result<(), ZipError> {
        let exclude = self.exclude.unwrap_or_default();
        let reader = Cursor::new(self.bytes);
        let mut archive = ZipArchive::new(reader)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let file_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            if exclude.contains(&&file_path.as_path())
                || file_path.parent().is_some_and(|p| exclude.contains(&p))
            {
                continue;
            }

            let output = output.join(&file_path);
            if file_path.is_dir() {
                fs::create_dir_all(output)?;
            } else {
                if let Some(p) = output.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }

                let mut outfile = fs::File::create(&output)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }
}
