use anyhow::Result;
use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressBar};

#[async_trait]
pub trait Retrieve: Send {
    type Record: Send;

    async fn retrieve(progress: MultiProgress) -> Result<Vec<Self::Record>>;
}

mod http;

pub(crate) use http::RetrieveAndParseHttp;

pub mod ca_solar;
pub mod eia;
pub mod nj_solar;
