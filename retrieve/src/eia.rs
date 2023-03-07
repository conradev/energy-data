use async_trait::async_trait;

#[derive(Default)]
pub struct Retriever;

#[async_trait]
impl crate::RetrieveAndParseHttp for Retriever {
    type Parser = parse::eia::ArchiveParser;

    const PROGRESS_NAME: &'static str = "eia";
    const URL_STRING: &'static str = "https://www.eia.gov/electricity/data/eia861/zip/f8612021.zip";
}
