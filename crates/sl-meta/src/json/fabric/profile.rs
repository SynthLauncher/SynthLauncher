// ! wrapper for fabric's meta /v2/versions/loader/:game_version/:loader_version/profile/json endpoint
use serde::{Deserialize, Serialize};

use crate::json::{vanilla, version_manifest::VersionType, JavaClassName};
#[derive(Debug, Clone, Deserialize, Serialize)]
struct FabricLibrary {
    name: JavaClassName,
    url: String,
    sha1: Option<String>,
    size: Option<i32>,
}

impl FabricLibrary {
    fn into_vanilla_library(&self) -> vanilla::Library {
        let (directory, jar) = self.name.into_directory_and_jar();
        let url = format!("{}/{}/{}", self.url, directory.display(), jar);

        vanilla::Library {
            downloads: vanilla::LibraryDownload {
                artifact: Some(vanilla::Download {
                    path: Some(directory.join(jar)),
                    url,
                    sha1: self.sha1.clone(),
                    size: self.size,
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
pub struct FabricLoaderProfile {
    pub id: String,
    /// The parent profile ID (the game version)
    pub inherits_from: String,
    pub r#type: VersionType,
    /// TODO: change into a proper date format
    pub release_time: String,
    /// TODO: change into a proper date format
    pub time: String,
    main_class: String,
    arguments: vanilla::Arguments,
    libraries: Vec<FabricLibrary>,
}

impl FabricLoaderProfile {
    fn libraries(&self) -> Vec<vanilla::Library> {
        self.libraries
            .iter()
            .map(|lib| lib.into_vanilla_library())
            .collect()
    }

    /// Joins the FabricLoaderProfile with a vanilla::Client to create a new vanilla::Client.
    pub fn join_client(self, client: vanilla::Client) -> vanilla::Client {
        let fabric_libraries = self.libraries();
        let mut client = client;
        client.id = self.id;
        client.main_class = self.main_class;
        client.arguments = client.arguments.concat(self.arguments);

        let libraries = client.libraries.into_iter();
        let libraries = libraries.filter(|c| {
            !fabric_libraries
                .iter()
                .any(|l| l.name.is_same_type(&c.name))
        });

        let mut libraries = libraries.collect::<Vec<_>>();
        libraries.extend(fabric_libraries.into_iter());
        client.libraries = libraries;
        client
    }
}

/// Get a FabricLoaderProfile from the Fabric Meta API.
/// do_request is a function that takes a URL and returns a Vec<u8> or an error.
/// if the response isn't valid JSON, it panics
pub async fn get_loader_profile<F, E>(
    game_version: &str,
    loader_version: &str,
    do_request: impl AsyncFnOnce(&str) -> Result<Vec<u8>, E>,
) -> Result<FabricLoaderProfile, E> {
    let url = format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
        game_version, loader_version
    );

    let response = do_request(&url).await?;
    Ok(serde_json::from_slice(&response).expect("response is invalid json"))
}
