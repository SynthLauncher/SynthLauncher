use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutThemeProps {
    pub root_container_props: Option<Value>,
    pub container_props: Option<Value>,
    pub main_window_container_props: Option<Value>,
    pub content_container_props: Option<Value>,
    pub navbar_props: Option<NavbarThemeProps>,
    pub sidebar_theme_props: Option<SidebarThemeProps>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavbarThemeProps {
    pub root_container_props: Option<Value>,
    pub container_props: Option<Value>,
    pub minimize_button_props: Option<Value>,
    pub maximize_button_props: Option<Value>,
    pub close_button_props: Option<Value>,
    pub minimize_icon_props: Option<Value>,
    pub maximize_icon_props: Option<Value>,
    pub close_icon_props: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarThemeProps {
    pub root_container_props: Option<Value>,
    pub container_props: Option<Value>,
    pub sidebar_container_props: Option<SidebarContainerProps>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarContainerProps {
    pub tooltip_props: Option<Value>,
    pub tooltip_trigger_props: Option<Value>,
    pub tooltip_content_props: Option<Value>,
    pub sidebar_item_props: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarItemThemeProps {
    pub button_props: Option<Value>,
    pub icon_props: Option<Value>,
}