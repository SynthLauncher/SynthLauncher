use serde::Deserialize;
use sl_meta::minecraft::loaders::vanilla::JavaComponent;

pub mod installer;

#[derive(Debug, Deserialize)]
pub struct ManifestDownload {
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
pub struct JreDownload {
    pub manifest: ManifestDownload,
    pub version: JreVersion,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JreDownloads {
    pub java_runtime_alpha: Vec<JreDownload>,
    pub java_runtime_beta: Vec<JreDownload>,
    pub java_runtime_delta: Vec<JreDownload>,
    pub java_runtime_gamma: Vec<JreDownload>,
    pub java_runtime_gamma_snapshot: Vec<JreDownload>,
    pub jre_legacy: Vec<JreDownload>,
    pub minecraft_java_exe: Vec<JreDownload>,
}

impl JreDownloads {
    pub fn get_by_component(&self, component: &JavaComponent) -> &[JreDownload] {
        match component {
            JavaComponent::JavaRuntimeAlpha => &self.java_runtime_alpha,
            JavaComponent::JavaRuntimeBeta => &self.java_runtime_beta,
            JavaComponent::JavaRuntimeDelta => &self.java_runtime_delta,
            JavaComponent::JavaRuntimeGamma => &self.java_runtime_gamma,
            JavaComponent::JavaRuntimeGammaSnapshot => &self.java_runtime_gamma_snapshot,
            JavaComponent::JreLegacy => &self.jre_legacy,
            JavaComponent::MinecraftJavaExe => &self.minecraft_java_exe,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JreManifest {
    pub gamecore: JreDownloads,
    pub linux: JreDownloads,
    pub linux_i386: JreDownloads,
    pub mac_os: JreDownloads,
    pub mac_os_arm64: JreDownloads,
    pub windows_arm64: JreDownloads,
    pub windows_x64: JreDownloads,
    pub windows_x86: JreDownloads,
}

impl JreManifest {
    pub fn get_current_platform_download(&self) -> &JreDownloads {
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

    pub fn get_component_downloads(&self, component: &JavaComponent) -> &[JreDownload] {
        self.get_current_platform_download()
            .get_by_component(component)
    }
}
