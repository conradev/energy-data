use chrono::NaiveDate;
use schema::ca_solar::Installation;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(remote = "Installation")]
pub struct Definition {
    #[serde(rename = "Application Id")]
    pub application_id: String,
    #[serde(rename = "Service City")]
    pub city: Option<String>,
    #[serde(rename = "Service Zip")]
    pub zip_code: Option<String>,
    #[serde(rename = "Service County")]
    pub county: Option<String>,
    #[serde(rename = "App Received Date")]
    pub recieved_date: Option<NaiveDate>,
    #[serde(rename = "App Complete Date")]
    pub complete_date: Option<NaiveDate>,
    #[serde(rename = "App Approved Date")]
    pub approved_date: Option<NaiveDate>,
    #[serde(rename = "System Size DC")]
    pub system_size_dc: f32,
    #[serde(rename = "System Size AC")]
    pub system_size_ac: f32,
    #[serde(rename = "Installer Name")]
    pub installer: Option<String>,
    #[serde(rename = "Utility")]
    pub utility: String,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Mapping(#[serde(with = "Definition")] Installation);

impl Into<Installation> for Mapping {
    fn into(self) -> Installation {
        self.0
    }
}
