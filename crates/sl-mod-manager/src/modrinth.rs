use std::{fs::File, io::Write, path::PathBuf};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModrinthFile {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectDownload {
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthFile>,
}

pub async fn install_modrinth_file(id: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", id);

    let client = Client::new();

    let response = client.get(&url).send().await?;

    let versions = response.json::<Vec<ProjectDownload>>().await?;

    let file_url = &versions[0].files[0].url;

    println!("{:?}", file_url);

    let mut resp = client.get(file_url).send().await?;

    println!("{:?}", resp);
    let mut file = File::create(dest.join("some.jar"))?;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
