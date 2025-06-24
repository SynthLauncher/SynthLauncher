use std::fs;

use sl_meta::minecraft::loaders::quilt::profiles::{get_quilt_loader_profile, QuiltLoaderProfile};
use sl_utils::utils::errors::{BackendError, HttpError};

use crate::launcher::instance::Instance;

pub async fn install_quilt_loader(
    instance: &Instance,
    loader_version: Option<&str>,
) -> Result<QuiltLoaderProfile, BackendError> {
    let path = instance.dir_path().join("quilt.json");
    let make_req = async |url: &str| -> Result<Vec<u8>, HttpError> {
        let res = reqwest::get(url).await?;
        let bytes = res.bytes().await?;
        Ok(bytes.to_vec())
    };

    let profile = get_quilt_loader_profile::<_, HttpError>(
        &instance.game_info.version,
        loader_version,
        make_req,
    )
    .await?;
    let file = fs::File::create(&path)?;
    serde_json::to_writer_pretty(file, &profile)?;

    Ok(profile)
}
