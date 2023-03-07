use anyhow::Result;
use async_trait::async_trait;
use futures::io::AsyncReadExt;
use futures::stream::{self, TryStreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::io;
use tokio_util::compat::FuturesAsyncReadCompatExt;

pub trait RetrieveAndParseHttp: Send {
    type Parser: parse::ParseBuffer;

    const PROGRESS_NAME: &'static str = "";
    const URL_STRING: &'static str;
}

#[async_trait]
impl<T> crate::Retrieve for T
where
    T: RetrieveAndParseHttp,
{
    type Record = <<T as RetrieveAndParseHttp>::Parser as parse::ParseBuffer>::Record;

    async fn retrieve(progress: MultiProgress) -> Result<Vec<Self::Record>> {
        use parse::ParseBuffer;

        let client = reqwest::Client::default();
        let response = client.get(T::URL_STRING).send().await?;

        let download_progress = if let Some(len) = response.content_length() {
            progress.add(ProgressBar::new(len)).with_style(
                ProgressStyle::with_template("{prefix}: {msg} {bar} {bytes}/{total_bytes}")
                    .unwrap(),
            )
        } else {
            progress
                .add(ProgressBar::new_spinner())
                .with_style(ProgressStyle::with_template("{prefix}: {msg} {spinner}").unwrap())
        }
        .with_prefix(T::PROGRESS_NAME)
        .with_message("Downloading");

        let mut bytes = vec![];
        let mut body = download_progress.wrap_async_read(
            response
                .bytes_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                .into_async_read()
                .compat(),
        );
        io::copy(&mut body, &mut bytes).await?;

        download_progress.finish();

        let reader = std::io::Cursor::new(bytes);
        let records = <T as RetrieveAndParseHttp>::Parser::from_buffer(reader, progress.clone())?;

        Ok(records)
    }
}
