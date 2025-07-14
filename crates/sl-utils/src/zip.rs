#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, Write},
    path::Path,
};

use zip::{result::ZipError, ZipArchive, ZipWriter};

#[derive(Debug)]
pub struct ZipBuilder<W: Write + Seek> {
    directories: Vec<String>,
    inner: ZipWriter<W>,
}

impl<W: Write + Seek> ZipBuilder<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: ZipWriter::new(inner),
            directories: Vec::new(),
        }
    }

    /// finishes writing to the Zip archive returning any errors
    ///
    /// drop would do this automatically but this is a method to handle errors
    pub fn finish(self) -> Result<W, ZipError> {
        self.inner.finish()
    }

    fn append_dir_existing(&mut self, dir_path_in_archive: &Path) -> Result<(), ZipError> {
        let name = zip::unstable::path_to_string(dir_path_in_archive);
        let name = name.to_string();

        if !self.directories.contains(&name) {
            let options = zip::write::SimpleFileOptions::default();
            self.inner.add_directory(&name, options)?;
            self.directories.push(name);
        }

        Ok(())
    }

    fn append_path_all(&mut self, path: &Path) -> Result<(), ZipError> {
        let mut current = Some(path);
        while let Some(path) = current {
            self.append_dir_existing(path)?;
            current = path.parent();
        }

        Ok(())
    }

    /// Recurisvely appends a direcotry to the archive
    pub fn append_directory_recursive<P: AsRef<Path>, PA: AsRef<Path>>(
        &mut self,
        path: P,
        path_in_archive: PA,
    ) -> Result<(), ZipError> {
        let path_in_archive = path_in_archive.as_ref();
        let path = path.as_ref();

        self.append_path_all(path_in_archive)?;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let e_type = entry.file_type()?;
            let e_name = entry.file_name();
            let e_path = entry.path();

            let e_path_in_archive = path_in_archive.join(e_name);
            if e_type.is_dir() {
                self.append_directory_recursive(&e_path, &e_path_in_archive)?
            } else {
                self.append_file(&e_path, &e_path_in_archive)?
            }
        }
        Ok(())
    }

    /// Appends a file with a given path to the archive, taking in a path in the archive
    pub fn append_file<P: AsRef<Path>, PA: AsRef<Path>>(
        &mut self,
        file_path: P,
        path_in_archive: PA,
    ) -> Result<(), ZipError> {
        let file_path = file_path.as_ref();
        let path_in_archive = path_in_archive.as_ref();

        let name = zip::unstable::path_to_string(path_in_archive);
        if let Some(parent) = path_in_archive.parent() {
            self.append_path_all(parent)?;
        }

        let file = File::open(file_path)?;
        let prems = file.metadata()?.permissions();
        _ = prems;

        let options = zip::write::SimpleFileOptions::default();
        #[cfg(unix)]
        let options = options.unix_permissions(prems.mode());

        self.inner.start_file(name, options)?;

        let mut reader = BufReader::new(file);
        std::io::copy(&mut reader, self.inner.by_ref())?;
        Ok(())
    }
}

pub struct ZipExtractor<'a, R: Read + Seek> {
    reader: R,
    exclude: Option<&'a [&'a Path]>,
}

impl<'a, R: Read + Seek> ZipExtractor<'a, R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            exclude: None,
        }
    }

    pub fn exclude(mut self, exclude: &'a [&'a Path]) -> Self {
        self.exclude = Some(exclude);
        self
    }

    // TODO: make async with the help of tokio_utils
    pub fn extract(self, output: &Path) -> Result<(), ZipError> {
        let exclude = self.exclude.unwrap_or_default();
        let mut archive = ZipArchive::new(self.reader)?;

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
            if file.is_dir() {
                fs::create_dir_all(output)?;
            } else {
                if let Some(p) = output.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }

                // TODO: Make this async
                let mut outfile = fs::File::create(&output)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }
}
