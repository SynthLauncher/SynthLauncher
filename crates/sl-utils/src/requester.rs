use std::{path::Path, time::Duration};

use bytes::Bytes;
use reqwest::{header::HeaderValue, Client, Response};
use tokio::{io::AsyncWriteExt, time::sleep};
use tokio_stream::StreamExt;
use url::Url;

use crate::{
    errors::HttpError,
    log,
    progress::{ProgressReport, ProgressSender},
};

pub struct RequestBuilder<'a> {
    requester: &'a Requester,
    retries: u32,
    retry_timeout: Duration,
    progress_sender: Option<&'a ProgressSender<'a>>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(requester: &'a Requester) -> Self {
        Self {
            requester,
            retries: 3,
            retry_timeout: Duration::from_secs(1),
            progress_sender: None,
        }
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn retry_timeout(mut self, retry_timeout: Duration) -> Self {
        self.retry_timeout = retry_timeout;
        self
    }

    /// Set the progress sender for the request.
    pub fn progress(mut self, progress_sender: Option<&'a ProgressSender<'a>>) -> Self {
        self.progress_sender = progress_sender;
        self
    }

    pub async fn download(&self, url: &str) -> Result<Bytes, HttpError> {
        self.requester
            .download(url, self.retries, self.retry_timeout)
            .await
    }

    pub async fn download_to(&self, url: &str, path: &Path) -> Result<(), HttpError> {
        self.requester
            .download_to(
                url,
                path,
                self.retries,
                self.retry_timeout,
                self.progress_sender,
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("SynthLauncher/1.0")
                .build()
                .expect("Failed to create a HTTP Client"),
        }
    }

    pub const fn client(&self) -> &Client {
        &self.client
    }

    pub fn builder(&self) -> RequestBuilder<'_> {
        RequestBuilder::new(&self)
    }

    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let parsed = Url::parse(url).expect("Invalid URL");
        let mut builder = self.client.get(url);

        if let Some(domain) = parsed.domain() {
            if domain == "api.curseforge.com" {
                builder = builder.header(
                    "x-api-key",
                    HeaderValue::from_static(
                        "$2a$10$/Dc9lilNTw0EvobjzoQLWu7zJpqX38hahG/jugi41F39z08R1rMZC",
                    ),
                );
            }
        }

        builder.send().await
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, HttpError> {
        Ok(self.get(url).await?.json::<T>().await?)
    }
}

impl Requester {
    async fn retry<T, F, Fut>(
        &self,
        mut f: F,
        max_retries: u32,
        delay: Duration,
        on_retry: impl Fn(u32),
    ) -> Result<T, HttpError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, HttpError>>,
    {
        let mut attempts = 0;
        loop {
            match f().await {
                Ok(val) => return Ok(val),
                Err(e) => {
                    attempts += 1;
                    on_retry(attempts);
                    if attempts >= max_retries {
                        return Err(e);
                    }
                    sleep(delay).await;
                }
            }
        }
    }

    async fn download_to_inner(
        &self,
        url: &str,
        path: &Path,
        mut progress_sender: Option<&ProgressSender<'_>>,
    ) -> Result<(), HttpError> {
        let response = self.get(url).await?;
        if !response.status().is_success() {
            return Err(HttpError::Status(response.status()));
        }

        let mut file = tokio::fs::File::create(&path).await?;
        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        let progress = ProgressReport::new(url, total_size, downloaded);
        if let Some(ref mut tx) = progress_sender {
            tx.send(progress);
        }

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if let Some(ref mut tx) = progress_sender {
                tx.send(progress.with_current(downloaded));
            }
        }

        if let Some(ref mut tx) = progress_sender {
            tx.send(progress.with_current(total_size));
            tx.stop_url(url);
        }

        Ok(())
    }

    async fn download(
        &self,
        url: &str,
        max_retries: u32,
        delay: Duration,
    ) -> Result<Bytes, HttpError> {
        self.retry(
            || {
                let url = url.to_string();
                let client = self.clone();
                async move {
                    let res = client.get(&url).await?;
                    if !res.status().is_success() {
                        return Err(HttpError::Status(res.status()));
                    }

                    let bytes = res.bytes().await?;
                    Ok(bytes)
                }
            },
            max_retries,
            delay,
            |attempt| log!("Retrying '{}', attempt {}", url, attempt),
        )
        .await
    }

    async fn download_to<'a>(
        &self,
        url: &str,
        path: &Path,
        max_retries: u32,
        delay: Duration,
        progress_sender: Option<&'a ProgressSender<'a>>,
    ) -> Result<(), HttpError> {
        let mut attempts = 0;
        // Generics sucks when it come to async
        loop {
            match self.download_to_inner(url, path, progress_sender).await {
                Ok(val) => return Ok(val),
                Err(e) => {
                    attempts += 1;
                    log!("Retrying '{}', attempt {}", url, attempts);
                    if attempts >= max_retries {
                        return Err(e);
                    }
                    sleep(delay).await;
                }
            }
        }
    }
}
