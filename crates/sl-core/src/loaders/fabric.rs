use std::{fs, path::Path};

use sl_meta::minecraft::loaders::fabric::profile::{get_loader_profile, FabricLoaderProfile};
use sl_utils::utils::errors::{BackendError, HttpError};

use crate::launcher::instances::metadata::InstanceMetadata;

pub async fn install_fabric_loader(
    instance: &InstanceMetadata,
    output_loader_json_path: &Path,
    loader_version: Option<&str>,
) -> Result<FabricLoaderProfile, BackendError> {
    let path = output_loader_json_path;
    let make_req = async |url: &str| -> Result<Vec<u8>, HttpError> {
        let res = reqwest::get(url).await?;
        let bytes = res.bytes().await?;
        Ok(bytes.to_vec())
    };

    let profile =
        get_loader_profile::<_, HttpError>(&instance.game_metadata.version, loader_version, make_req)
            .await?;

    let file = fs::File::create(&path)?;
    serde_json::to_writer_pretty(file, &profile)?;

    Ok(profile)
}
