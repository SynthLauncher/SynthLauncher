use serde::{Deserialize, Serialize};
use sl_utils::{dlog, utils::errors::BackendError};

use crate::{launcher::themes::{layout::LayoutThemeProps, store_page::StorePageThemeProps}, THEMES_DIR};

pub mod store_page;
pub mod layout;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub store_page: Option<StorePageThemeProps>,
    pub layout: Option<LayoutThemeProps>
}

pub fn get_themes() -> Result<Vec<Theme>, BackendError> {
    let mut themes = Vec::new();
    for entry in std::fs::read_dir(THEMES_DIR.as_path())? {
        let entry = entry?;
        let path = entry.path();
        let json = std::fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&json)?;
        themes.push(theme);
    }

    dlog!("test");
    dlog!("{:#?}", themes);

    Ok(themes)
}
