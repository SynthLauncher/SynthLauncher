use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: usize,
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    #[serde(default)]
    pub map_to_resources: bool,
    pub objects: HashMap<String, AssetObject>,
}
