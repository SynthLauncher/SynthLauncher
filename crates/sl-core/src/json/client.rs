use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use futures::{stream::FuturesUnordered, StreamExt};
use sha1::{Digest, Sha1};
use sl_meta::json::vanilla::{AssetIndex, AssetObject, Client, Download, Library};
use sl_utils::utils::{
    self,
    errors::{BackendError, DownloadError},
    zip::ZipExtractor,
};

use crate::{ASSETS_DIR, LIBS_DIR, TEMP_CLIENT};

#[inline(always)]
fn verify_data(file: &mut File, sha1: &str) -> bool {
    let mut hasher = Sha1::new();

    let Ok(_) = std::io::copy(file, &mut hasher) else {
        return false;
    };

    let hash = hasher.finalize();
    if hash.as_slice() != sha1.as_bytes() {
        return false;
    }

    true
}

#[inline(always)]
async fn download_and_verify(download: &Download, path: &Path) -> Result<(), DownloadError> {
    if File::open(path).is_ok_and(|mut f| {
        if let Some(sha1) = &download.sha1 {
            verify_data(&mut f, sha1)
        } else {
            true
        }
    }) {
        return Ok(());
    }

    let data = utils::download::get_as_bytes(&download.url).await?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, &data)?;
    Ok(())
}

async fn download_and_read_file(download: &Download, path: &Path) -> Result<Bytes, DownloadError> {
    let full_path = if let Some(ref child) = download.path {
        &path.join(child)
    } else {
        path
    };

    download_and_verify(download, full_path).await?;
    Ok(Bytes::from(
        fs::read(full_path).expect("get_download_in: failed to read downloaded file"),
    ))
}

async fn download_to(download: &Download, path: &Path) -> Result<(), DownloadError> {
    let full_path = if let Some(ref child) = download.path {
        &path.join(child)
    } else {
        path
    };

    download_and_verify(download, full_path).await
}

#[inline(always)]
async fn download_futures<T, F, R, I>(to_download: I, download_max: usize, download: F) -> Vec<R>
where
    I: Iterator<Item = T>,
    F: AsyncFn(T) -> R,
{
    let (size_0, size_1) = to_download.size_hint();
    let size_hint = size_1.unwrap_or(size_0);

    let mut outputs = Vec::with_capacity(size_hint.min(10));
    let mut futures = FuturesUnordered::new();

    for item in to_download {
        futures.push(download(item));
        if futures.len() == download_max {
            let next = futures.next().await.unwrap();
            outputs.push(next);
        }
    }

    while let Some(future) = futures.next().await {
        outputs.push(future);
    }

    outputs
}

async fn install_assets(
    // client: &Client
) -> Result<(), DownloadError> {
    let id = (*TEMP_CLIENT).lock().await.as_ref().unwrap().assets.clone();
    let indexes_dir = ASSETS_DIR.join("indexes");
    let indexes_path = indexes_dir.join(format!("{}.json", id));

    let download = download_and_read_file(&(*TEMP_CLIENT).lock().await.as_ref().unwrap().asset_index, &indexes_path).await?;

    let index: AssetIndex = serde_json::from_slice(&download).unwrap();
    let objects = index.objects;

    let download_object = async |object: AssetObject| -> Result<(), DownloadError> {
        let dir_name = &object.hash[0..2];
        let dir = ASSETS_DIR.join("objects").join(dir_name);
        let path = dir.join(&object.hash);

        if path.exists() {
            return Ok(());
        }

        fs::create_dir_all(&dir)?;
        let data = utils::download::get_as_bytes(&format!(
            "https://resources.download.minecraft.net/{dir_name}/{}",
            object.hash
        ))
        .await?;

        fs::write(path, data)?;
        Ok(())
    };

    let iter = objects.into_iter();
    let iter = iter.map(|(_, object)| object);
    let outputs = download_futures(iter, 20, download_object).await;
    for (i, output) in outputs.into_iter().enumerate() {
        if let Err(err) = output {
            println!("Failed to download object indexed {i}: {err:?}");
            return Err(err);
        }
    }

    println!("Downloaded assets for {}", id);
    Ok(())
}

async fn install_libs(
    path: PathBuf
) -> Result<(), BackendError> {
    println!("Downloading libraries...");

    let download_lib = async |lib: &Library| -> Result<(), BackendError> {
        if let Some(ref artifact) = lib.downloads.artifact {
            download_to(artifact, &LIBS_DIR).await?;
        }

        // !!! This needs to be fixed!!!
        // if let Some(native) = lib.native_from_platform() {
        //     let bytes = download_and_read_file(native, &LIBS_DIR).await?;

        //     if let Some(ref extract_rules) = lib.extract {
        //         let natives_dir = path.join(".natives");

        //         let exclude = extract_rules.exclude.as_deref().unwrap_or_default();
        //         let paths = exclude.iter().map(PathBuf::as_path).collect::<Vec<_>>();
        //         let zip = ZipExtractor::new(&bytes).exclude(&paths);

        //         zip.extract(&natives_dir)?;
        //     }
        // }
        Ok(())
    };

    let outputs = download_futures((*TEMP_CLIENT).lock().await.as_ref().unwrap().libraries(), 5, download_lib).await;
    for (i, output) in outputs.into_iter().enumerate() {
        if let Err(err) = output {
            println!("Failed to download library indexed {i}: {err:?}");
            return Err(err);
        }
    }

    println!("Done downloading libraries");
    Ok(())
}

pub async fn install_client(
    path: PathBuf,
) -> Result<(), BackendError> {
    install_assets().await?;
    install_libs(path.clone()).await?;

    let client_path = path.join("client.jar");

    download_to(&(*TEMP_CLIENT).lock().await.take().unwrap().downloads.client, &client_path).await?;

    Ok(())
}
