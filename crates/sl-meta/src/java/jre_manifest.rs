use std::collections::HashMap;

use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Deserialize, Display, EnumString, Clone)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum JreManifestDownloadType {
    JavaRuntimeAlpha,
    JavaRuntimeBeta,
    JavaRuntimeDelta,
    JavaRuntimeGamma,
    JavaRuntimeGammaSnapshot,
    JreLegacy,
    MinecraftJavaExe,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub sha1: String,
    pub url: String,
    pub size: usize,
}

#[derive(Debug, Deserialize)]
pub struct JreVersion {
    pub name: String,
    pub released: String,
}

#[derive(Debug, Deserialize)]
pub struct JreManifestDownload {
    pub manifest: Manifest,
    pub version: JreVersion,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OsDownload {
    pub java_runtime_alpha: Vec<JreManifestDownload>,
    pub java_runtime_beta: Vec<JreManifestDownload>,
    pub java_runtime_delta: Vec<JreManifestDownload>,
    pub java_runtime_gamma: Vec<JreManifestDownload>,
    pub java_runtime_gamma_snapshot: Vec<JreManifestDownload>,
    pub jre_legacy: Vec<JreManifestDownload>,
    pub minecraft_java_exe: Vec<JreManifestDownload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JreManifest {
    pub gamecore: OsDownload,
    pub linux: OsDownload,
    pub linux_i386: OsDownload,
    pub mac_os: OsDownload,
    pub mac_os_arm64: OsDownload,
    pub windows_arm64: OsDownload,
    pub windows_x64: OsDownload,
    pub windows_x86: OsDownload,
}

impl JreManifest {
    pub fn get_current_platform_download(&self) -> &OsDownload {
        use super::Platform::*;

        match super::Platform::detect() {
            Linux => &self.linux,
            LinuxI386 => &self.linux_i386,
            MacOs => &self.mac_os,
            MacOsArm64 => &self.mac_os_arm64,
            WindowsArm64 => &self.windows_arm64,
            WindowsX86 => &self.windows_x86,
            WindowsX64 => &self.windows_x64,
        }
    }

    pub fn get_jre_manifest_download(
        &self,
        download: &JreManifestDownloadType,
    ) -> &Vec<JreManifestDownload> {
        let platform_download = self.get_current_platform_download();

        match download {
            JreManifestDownloadType::JavaRuntimeAlpha => &platform_download.java_runtime_alpha,
            JreManifestDownloadType::JavaRuntimeBeta => &platform_download.java_runtime_beta,
            JreManifestDownloadType::JavaRuntimeDelta => &platform_download.java_runtime_delta,
            JreManifestDownloadType::JavaRuntimeGamma => &platform_download.java_runtime_gamma,
            JreManifestDownloadType::JavaRuntimeGammaSnapshot => &platform_download.java_runtime_gamma_snapshot,
            JreManifestDownloadType::JreLegacy => &platform_download.jre_legacy,
            JreManifestDownloadType::MinecraftJavaExe => &platform_download.minecraft_java_exe,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JavaFile {
    pub r#type: String,
    pub target: Option<String>,
    pub downloads: Option<JavaFileDownloads>,
}

#[derive(Debug, Deserialize)]
pub struct JavaFileDownload {
    pub sha1: String,
    pub url: String,
    pub size: usize,
}

#[derive(Debug, Deserialize)]
pub struct JavaFileDownloads {
    pub lzma: Option<JavaFileDownload>,
    pub raw: Option<JavaFileDownload>,
}

#[derive(Debug, Deserialize)]
pub struct JavaFiles {
    pub files: HashMap<String, JavaFile>,
}

impl JavaFiles {
    pub fn java_file_by_type<'a>(
        &'a self,
        r#type: &'a str,
    ) -> impl Iterator<Item = (&'a String, &'a JavaFile)> + 'a {
        self.files
            .iter()
            .filter(move |(_, file)| file.r#type == r#type)
    }
}
