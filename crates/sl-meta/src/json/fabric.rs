use serde::Deserialize;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fabric {
    pub main_class: String,
    pub arguments: Option<Arguments>,
    pub libraries: Vec<Library>,
}

#[derive(Debug, Deserialize)]
pub struct Arguments {
    pub jvm: Option<Vec<String>>,
    pub game: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub name: String,
    pub url: String,
}

impl Library {
    fn split_name(&self) -> (&str, &str, &str) {
        let mut parts = self.name.split(':');
        let group = parts.next().unwrap();
        let artifact = parts.next().unwrap();
        let version = parts.next().unwrap();

        return (group, artifact, version);
    }

    pub fn get_path(&self) -> PathBuf {
        let (group, artifact, version) = self.split_name();

        let est_size = group.len() + artifact.len() * 2 + version.len() * 2 + 5;
        let mut path = PathBuf::with_capacity(est_size);

        path.push(group.replace('.', MAIN_SEPARATOR_STR));
        path.push(artifact);
        path.push(version);
        path.push(format!("{}-{}.jar", artifact, version));
        path
    }

    pub fn get_url(&self) -> String {
        let (group, artifact, version) = self.split_name();
        format!(
            "{}/{}/{}/{}/{}-{}.jar",
            self.url.trim_end_matches('/'),
            group.replace('.', "/"),
            artifact,
            version,
            artifact,
            version
        )
    }
}
