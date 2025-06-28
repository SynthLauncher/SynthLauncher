use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreCategorySelectorThemeProps {
    pub category_list_props: Option<Value>, 
    pub category_trigger_props: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorePageThemeProps {
    pub container_props: Option<Value>,
    pub store_category_selector: Option<StoreCategorySelectorThemeProps>,
    pub input_props: Option<Value>,
}
