use sl_utils::zip::ZipBuilder;

use std::{io::{Seek, Write}, path::PathBuf};
use zip::result::ZipError;

/// an Instance Exporter to a writer, exports in Zip format
#[derive(Debug)]
pub struct InstanceExporter<'a, W: Write + Seek> {
    inner: ZipBuilder<W>,
    instance_root: PathBuf,
    exclude_list: Vec<&'a str>,
}

impl<'a, W: Write + Seek> InstanceExporter<'a, W> {
    pub(super) fn new(output: W, instance_root: PathBuf) -> Self {
        Self {
            instance_root,
            inner: ZipBuilder::new(output),
            exclude_list: Vec::new(),
        }
    }

    /// Excludes a given file name, relative to the instance root from the exporting
    pub fn exclude(&mut self, file_name: &'a str) -> &mut Self {
        self.exclude_list.push(file_name);
        self
    }

    /// Exports the instance returning the export results
    pub fn export(mut self) -> Result<W, ZipError> {
        for entry in std::fs::read_dir(&self.instance_root)? {
            let entry = entry?;
            let name = entry.file_name();
            let ty = entry.file_type()?;
            if name.as_encoded_bytes().starts_with(&[b'.'])
                || self
                    .exclude_list
                    .iter()
                    .any(|s| s.as_bytes() == name.as_encoded_bytes())
            {
                continue;
            }

            let path_in_instance = self.instance_root.join(&name);

            if ty.is_dir() {
                self.inner
                    .append_directory_recursive(path_in_instance, name)?;
            } else {
                self.inner.append_file(path_in_instance, name)?
            }
        }
        self.inner.finish()
    }
}
