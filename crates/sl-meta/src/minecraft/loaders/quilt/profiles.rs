use serde::{Deserialize, Serialize};

use crate::minecraft::{
    loaders::vanilla::{Client, Download, Library, LibraryDownload},
    JavaClassName,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuiltLibrary {
    pub name: JavaClassName,
    pub url: String,
}

impl QuiltLibrary {
    fn into_vanilla_library(&self) -> Library {
        let (directory, jar) = self.name.into_directory_and_jar();
        let url = format!("{}/{}/{}", self.url, directory.display(), jar);

        Library {
            downloads: LibraryDownload {
                artifact: Some(Download {
                    path: Some(directory.join(jar)),
                    url,
                    sha1: None,
                    size: None,
                }),
                classifiers: None,
            },
            rules: None,
            extract: None,
            natives: None,
            name: self.name.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuiltLoaderProfile {
    pub id: String,
    pub inherits_from: String,
    pub main_class: String,
    pub libraries: Vec<QuiltLibrary>,
}

impl QuiltLoaderProfile {
    fn libraries(&self) -> Vec<Library> {
        self.libraries
            .iter()
            .map(|lib| lib.into_vanilla_library())
            .collect()
    }

    pub fn join_client(self, client: Client) -> Client {
        let quilt_libraries = self.libraries();
        let mut client = client;
        client.id = self.id;
        client.main_class = self.main_class;

        let libraries = client.libraries.into_iter();
        let libraries =
            libraries.filter(|c| !quilt_libraries.iter().any(|l| l.name.is_same_type(&c.name)));

        let mut libraries = libraries.collect::<Vec<_>>();
        libraries.extend(quilt_libraries.into_iter());
        client.libraries = libraries;
        client
    }
}

pub async fn get_quilt_loader_profile<F, E>(
    game_version: &str,
    loader_version: &str,
    do_request: F,
) -> Result<QuiltLoaderProfile, E>
where
    F: AsyncFn(&str) -> Result<Vec<u8>, E> + Copy,
    E: From<std::io::Error>,
{
    let url = format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
        game_version, loader_version
    );

    let response = do_request(&url).await?;
    Ok(serde_json::from_slice(&response).expect("response is invalid json"))
}
