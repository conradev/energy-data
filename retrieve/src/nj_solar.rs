use async_trait::async_trait;

#[derive(Default)]
pub struct Retriever;

#[async_trait]
impl crate::RetrieveAndParseHttp for Retriever {
    type Parser = parse::nj_solar::Parser;

    const PROGRESS_NAME: &'static str = "nj_solar";
    const URL_STRING: &'static str =
        "https://solarreports.s3.amazonaws.com/DATA+-+INSTALLED+-+SEPTEMBER+2022.xlsx";
}
