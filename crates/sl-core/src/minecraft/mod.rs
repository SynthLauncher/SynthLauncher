use std::{
    io::Cursor,
    path::{Path, PathBuf},
    time::Instant,
};

use bytes::Bytes;
use futures::StreamExt;

use sl_meta::minecraft::loaders::vanilla::{AssetIndex, AssetObject, Client, Download, Library};
use sl_utils::{
    elog,
    errors::{BackendError, HttpError},
    log,
    progress::{ProgressReceiver, ProgressSender},
    requester::Requester,
    zip::ZipExtractor,
};

pub(crate) mod minecraft_version;
pub(crate) mod version_manifest;

// TODO: Implement verify_data function that is fast enough, this one is really slow so i removed it and replaced it with verifying size
// #[inline(always)]
// async fn verify_data(file: &mut tokio::fs::File, sha1: &str) -> bool {
//     let mut hasher = Sha1::new();
//     let mut buffer = [0u8; 8192];

//     loop {
//         match file.read(&mut buffer).await {
//             Ok(0) => break, // EOF
//             Ok(n) => hasher.update(&buffer[..n]),
//             Err(_) => return false,
//         }
//     }

//     let hash = hasher.finalize();
//     hash.as_slice() == sha1.as_bytes()
// }

/// Downloads and verifies the integrity of a given client Download entry
#[inline]
async fn download_and_verify(
    requester: &Requester,
    prog_sender: &ProgressSender<'_>,
    download: &Download,
    path: &Path,
) -> Result<(), HttpError> {
    if let Ok(f) = tokio::fs::File::open(path).await {
        let valid = match download.size {
            Some(size) => {
                let f_metadata = f.metadata().await;
                let f_size = f_metadata.map(|m| m.len()).ok();
                f_size.is_none_or(|f_size| f_size == size as u64)
            }
            None => true,
        };

        if valid {
            return Ok(());
        }
    }

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    requester
        .builder()
        .progress(Some(prog_sender))
        .download_to(&download.url, &path)
        .await?;

    Ok(())
}

async fn download_and_read_file(
    requester: &Requester,
    prog_sender: &ProgressSender<'_>,
    download: &Download,
    path: &Path,
) -> Result<Bytes, HttpError> {
    let full_path = if let Some(ref child) = download.path {
        &path.join(child)
    } else {
        path
    };

    download_and_verify(requester, prog_sender, download, full_path).await?;
    Ok(Bytes::from(tokio::fs::read(full_path).await?))
}

async fn download_to(
    requester: &Requester,
    prog_sender: &ProgressSender<'_>,
    download: &Download,
    path: &Path,
) -> Result<(), HttpError> {
    if download.url.is_empty() {
        return Ok(());
    }

    let full_path = if let Some(ref child) = download.path {
        &path.join(child)
    } else {
        path
    };

    download_and_verify(requester, prog_sender, download, full_path).await?;
    Ok(())
}

// This could be made faster maybe
async fn install_assets(
    requester: &Requester,
    prog_recv: &ProgressReceiver,
    assets_root_path: &Path,
    client: &Client,
) -> Result<(), BackendError> {
    const MAX_DOWNLOADS: usize = 5;
    log!("Downloading assets!");
    let sender = prog_recv.begin_sending("Downloading assets");
    let assets = &client.assets;

    let indexes_dir = assets_root_path.join("indexes");
    let objects_dir = assets_root_path.join("objects");

    let indexes_path = indexes_dir.join(format!("{}.json", assets));

    let download =
        download_and_read_file(requester, &sender, &client.asset_index, &indexes_path).await?;

    let index: AssetIndex = serde_json::from_slice(&download)?;
    let objects = index.objects;

    let download_object = async |object: AssetObject| -> Result<(), HttpError> {
        let dir_name = &object.hash[0..2];
        let dir = objects_dir.join(dir_name);

        let path = dir.join(&object.hash);

        if path.exists() {
            return Ok(());
        }

        tokio::fs::create_dir_all(&dir).await?;

        requester
            .builder()
            .progress(Some(&sender))
            .download_to(
                &format!(
                    "https://resources.download.minecraft.net/{dir_name}/{}",
                    object.hash
                ),
                &path,
            )
            .await?;

        Ok(())
    };

    let len = objects.len();
    let iter = objects.into_iter();
    let iter = iter.map(|(_, object)| object);

    let mut downloads = Vec::with_capacity(len);
    for obj in iter {
        downloads.push(download_object(obj));
    }

    let results = futures::stream::iter(downloads)
        .buffer_unordered(MAX_DOWNLOADS)
        .collect::<Vec<_>>()
        .await;
    for (i, output) in results.into_iter().enumerate() {
        if let Err(err) = output {
            elog!("Failed to download object indexed {i}: {err:?}");
            return Err(BackendError::HttpError(err));
        }
    }

    log!("Downloaded assets for {}", assets);
    Ok(())
}

async fn install_libs(
    requester: &Requester,
    prog_recv: &ProgressReceiver,
    client: &Client,
    libs_dir: &Path,
    path: &Path,
) -> Result<(), BackendError> {
    const MAX_DOWNLOADS: usize = 10;
    log!("Downloading libraries...");
    let sender = prog_recv.begin_sending("Downloading libraries");

    let download_lib = async move |lib: &Library| -> Result<(), BackendError> {
        if let Some(ref artifact) = lib.downloads.artifact {
            download_to(requester, &sender, artifact, libs_dir).await?;
        }

        if let Some(native) = lib.native_from_platform() {
            // FIXME: this is so terrible just download and return a reader at least
            let bytes = download_and_read_file(requester, &sender, native, libs_dir).await?;

            if let Some(ref extract_rules) = lib.extract {
                let natives_dir = path.join(".natives");

                let exclude = extract_rules.exclude.as_deref().unwrap_or_default();
                let paths = exclude.iter().map(PathBuf::as_path).collect::<Vec<_>>();

                let cursor = Cursor::new(bytes);
                let zip = ZipExtractor::new(cursor).exclude(&paths);

                zip.extract(&natives_dir)?;
            }
        }
        Ok(())
    };

    let len = client.libraries_len_hint();
    let mut downloads = Vec::with_capacity(len);

    for lib in client.libraries() {
        downloads.push(download_lib(lib));
    }
    let results = futures::stream::iter(downloads)
        .buffer_unordered(MAX_DOWNLOADS)
        .collect::<Vec<_>>()
        .await;
    for (i, output) in results.into_iter().enumerate() {
        if let Err(err) = output {
            elog!("Failed to download library indexed {i}: {err:?}");
            return Err(err);
        }
    }

    log!("Done downloading libraries");
    Ok(())
}

pub(crate) async fn install_client(
    requester: &Requester,
    prog_recv: &ProgressReceiver,
    client: &Client,
    client_jar_path: &Path,
    instance_path: &Path,
    assets_root_path: &Path,
    libs_root_path: &Path,
) -> Result<(), BackendError> {
    let start = Instant::now();
    install_assets(requester, prog_recv, assets_root_path, client).await?;
    println!("Assets download time: {:?}", start.elapsed());

    install_libs(requester, prog_recv, client, libs_root_path, instance_path).await?;

    log!("Downloading {}", client_jar_path.display());
    let client_sender = prog_recv.begin_sending("Downloading minecraft client");
    download_to(
        requester,
        &client_sender,
        &client.downloads.client,
        &client_jar_path,
    )
    .await?;
    log!("Done downloading {}", client_jar_path.display());

    Ok(())
}
