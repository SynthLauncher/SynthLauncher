use sl_core::{environment::LauncherEnv, instances::content_manager::{ContentList, ContentManager}};
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn get_content_list(
    instance_name: &str,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<ContentList, String> {
    let env = launcher_env.read().await;
    let (_, path) = env.instances().get_existing(&instance_name).map_err(|e| e.to_string())?;   
    let mut content_manager = ContentManager::new(&path.parent().unwrap());
    let list = content_manager.load_content_list().await.map_err(|e| e.to_string())?;
    Ok(list)
}

