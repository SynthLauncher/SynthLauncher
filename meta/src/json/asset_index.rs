use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: i32,
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[cfg(test)]
mod tests {
    use crate::json::asset_index::AssetIndex;

    use super::AssetObject;

    #[test]
    fn deserialize_asset_object() {
        let json = r#"
        {
            "hash": "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356",
            "size": 9101
        }
        "#;

        let asset: AssetObject = serde_json::from_str(json).unwrap();
        assert_eq!(asset.hash, "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356");
        assert_eq!(asset.size, 9101);
    }

    #[test]
    fn deserialize_asset_index() {
        let json = r#"
        {
            "objects": {
                "icons/icon_128x128.png": {
                    "hash": "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356",
                    "size": 9101
                },
                "icons/icon_16x16.png": {
                    "hash": "5ff04807c356f1beed0b86ccf659b44b9983e3fa",
                    "size": 781
                },
                "icons/icon_256x256.png": {
                    "hash": "8030dd9dc315c0381d52c4782ea36c6baf6e8135",
                    "size": 19642
                }
            }
        }
        "#;

        let asset_index: AssetIndex = serde_json::from_str(json).unwrap();
        assert_eq!(asset_index.objects.get("icons/icon_128x128.png").unwrap().hash, "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356");
        assert_eq!(asset_index.objects.get("icons/icon_128x128.png").unwrap().size, 9101);

        assert_eq!(asset_index.objects.get("icons/icon_16x16.png").unwrap().hash, "5ff04807c356f1beed0b86ccf659b44b9983e3fa");
        assert_eq!(asset_index.objects.get("icons/icon_16x16.png").unwrap().size, 781);
        
        assert_eq!(asset_index.objects.get("icons/icon_256x256.png").unwrap().hash, "8030dd9dc315c0381d52c4782ea36c6baf6e8135");
        assert_eq!(asset_index.objects.get("icons/icon_256x256.png").unwrap().size, 19642);
    }
}
