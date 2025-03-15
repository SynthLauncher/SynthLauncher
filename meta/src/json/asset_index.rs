use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: i32
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>
}
