use async_trait::async_trait;

#[derive(Default)]
pub struct Retriever;

#[async_trait]
impl crate::RetrieveAndParseHttp for Retriever {
    type Parser = parse::ca_solar::ArchiveParser;

    const PROGRESS_NAME: &'static str = "ca_solar";
    const URL_STRING: &'static str =
        "https://www.californiadgstats.ca.gov/download/interconnection_rule21_projects/Interconnected_Project_Sites_2022-10-31.zip";
}
