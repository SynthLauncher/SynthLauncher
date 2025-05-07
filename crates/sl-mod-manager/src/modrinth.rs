// use reqwest::Client;
// use serde::Deserialize;
// use urlencoding::encode;

// enum Loaders {
//     Fabric,
//     Forge,
//     NeoForge,
//     Quilt,
// }

// #[derive(Debug, Deserialize)]
// struct ModrinthSearchResult {
//     hits: Vec<ModProject>,
// }

// #[derive(Debug, Deserialize)]
// struct ModProject {
//     title: String,
//     slug: String,
//     downloads: u32,
//     description: Option<String>,
//     icon_url: Option<String>,
//     versions: Vec<String>
// }

// pub fn build_facets(loader_type: String, version: &str) -> String {
//     let mut facets = vec![];

//     facets.push(format!());

//     if !version.is_empty() {
//         facets.push(vec![format!("versions:{}", version)]);
//     }

//     let json = serde_json::to_string(&facets).unwrap();
//     encode(&json).to_string()
// }

// pub async fn search_modrinth(query: &str, sort_by: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();

//     let facets = build_facets(true, "1.17.1");

//     let url = format!(
//         "https://api.modrinth.com/v2/search?query={}&facets={}&index={}",
//         encode(query),
//         facets,
//         encode(sort_by)
//     );

//     let response = client.get(&url)
//             .send()
//             .await?
//             .json::<ModrinthSearchResult>()
//             .await?;

//     for project in response.hits {
//         println!(
//             "- {} ({}): {} downloads\n  {}\n  Icon: {}",
//             project.title,
//             project.slug,
//             project.downloads,
//             project.description.clone().unwrap_or("No description.".to_string()),
//             project.icon_url.clone().unwrap_or("No icon.".to_string())
//         );
//     }

//     Ok(())
// }
