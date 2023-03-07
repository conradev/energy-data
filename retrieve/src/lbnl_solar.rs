use async_trait::async_trait;

#[derive(Default)]
pub struct Retriever;

#[async_trait]
impl crate::RetrieveAndParseHttp for Retriever {
    type Parser = parse::ca_solar::Parser;

    const PROGRESS_NAME: &'static str = "lbnl_solar";
    const URL_STRING: &'static str = "https://emp.lbl.gov/sites/default/files/public_datafile.zip";
}
