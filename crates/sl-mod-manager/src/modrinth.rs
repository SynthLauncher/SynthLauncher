/* 
    This is not ready!
    !!! I was just messing with it!
*/

use reqwest::Client;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
struct ModrinthSearchResult {
    hits: Vec<ModProject>,
}

#[derive(Debug, Deserialize)]
struct ModProject {
    title: String,
    slug: String,
    downloads: u32,
    description: Option<String>,
    icon_url: Option<String>,
    // versions: Vec<String>
}

pub fn build_facets(project_type: &str, version: &str) -> String {
    let mut facet_groups = vec![];

    facet_groups.push(vec![format!("project_type:{}", project_type)]);

    if !version.is_empty() {
        facet_groups.push(vec![format!("versions:{}", version)]);
    }

    let json = serde_json::to_string(&facet_groups).unwrap();
    encode(&json).to_string()
}


pub async fn search_modrinth(query: &str, project_type: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let facets = build_facets(project_type, version);

    let url = format!(
        "https://api.modrinth.com/v2/search?query={}&facets={}",
        encode(query),
        facets,
    );

    let response = client.get(&url)
            .send()
            .await?
            .json::<ModrinthSearchResult>()
            .await?;

    for project in response.hits {
        println!(
            "- {} ({}): {} downloads\n  {}\n  Icon: {}\n URL: {}",
            project.title,
            project.slug,
            project.downloads,
            project.description.clone().unwrap_or("No description.".to_string()),
            project.icon_url.clone().unwrap_or("No icon.".to_string()),
            format!("https://modrinth.com/{}/{}", project_type, project.slug)
        );
    }

    Ok(())
}

// pub fn async get_mod() {

// }

// pub async fn add_mod() {

// }
