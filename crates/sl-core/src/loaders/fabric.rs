use std::{fs, path::Path};

use sl_meta::minecraft::loaders::fabric::profile::{get_loader_profile, FabricLoaderProfile};
use sl_utils::errors::{BackendError, HttpError};

pub async fn install_fabric_loader(
    minecraft_version: &str,
    output_loader_json_path: &Path,
    loader_version: &str,
) -> Result<FabricLoaderProfile, BackendError> {
    let path = output_loader_json_path;
    let make_req = async |url: &str| -> Result<Vec<u8>, HttpError> {
        let res = reqwest::get(url).await?;
        let bytes = res.bytes().await?;
        Ok(bytes.to_vec())
    };

    let profile =
        get_loader_profile::<_, HttpError>(minecraft_version, loader_version, make_req).await?;

    let file = fs::File::create(&path)?;
    serde_json::to_writer_pretty(file, &profile)?;

    Ok(profile)
}
