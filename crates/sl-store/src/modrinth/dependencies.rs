// use std::sync::Arc;

// use dashmap::DashMap;

// type ProjectId = String;
// type VersionId = String;

// #[derive(Debug, Clone)]
// pub struct ResolutionState {
//     pub resolved: Arc<DashMap<ProjectId, VersionId>>
// }

// impl ResolutionState {
//     pub fn new() -> Self {
//         Self {
//             resolved: Arc::new(DashMap::new()),
//         }
//     }
// }

// async fn pick_latest_version_id(
//     project_id: &str,
//     game_version: &str,
//     loader: &str,
// ) -> Result<String, BackendError> {
//     let versions = query_project_versions(project_id, Some(game_version), Some(loader)).await?;

//     let latest_id = versions[0].id.clone();
//     Ok(latest_id)
// }

// pub async fn resolve_mod(
//     state: &ResolutionState,
//     project_id: String,
//     version_id: String,
//     game_version: &str,
//     loader: &str,
// ) -> Result<(), BackendError> {
//     if state.resolved.contains_key(&project_id) {
//         return Ok(());a
//     }

//     state.resolved.insert(project_id.clone(), version_id.clone());

//     let version_info = query_project_version(&project_id, &version_id).await?;

//     let mut futures = FuturesUnordered::new();

//     for dep in &version_info.dependencies {
//         if dep.dependency_type != "required" {
//             continue;
//         }

//         let dep_vid = if let Some(vid) = &dep.version_id {
//             vid.clone()
//         } else {
//             pick_latest_version_id(&dep.project_id, game_version, loader).await?
//         };

//         futures.push(resolve_mod(
//             state,
//             dep.project_id.clone(),
//             dep_vid,
//             game_version,
//             loader,
//         ));
//     }

//     while let Some(res) = futures.next().await {
//         res?;
//     }

//     Ok(())
// }

// pub async fn install_mod_with_deps(
//     slug: &str,
//     version: &str,
//     instance_path: &Path,
// ) -> Result<(), BackendError> {
//     let project_type = ProjectType::Mod;

//     let state = ResolutionState::new();

//     resolve_mod(
//         &state,
//         slug.to_string(),
//         version.to_string(),
//         "1.20.1",
//         "fabric",
//     )
//     .await?;

//     let mut futures = FuturesUnordered::new();

//     for entry in state.resolved.iter() {
//         let proj_id = entry.key().clone();
//         let ver_id = entry.value().clone();
//         let instance_path = instance_path.to_owned();
//         let pt = project_type.clone();

//         futures.push(async move { install_project(&proj_id, &ver_id, &instance_path, pt).await });
//     }

//     while let Some(res) = futures.next().await {
//         res?;
//     }

//     Ok(())
// }