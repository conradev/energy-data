use chrono::NaiveDate;
use schema::nj_solar::Installation;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(remote = "Installation")]
struct InstallationDef {
    #[serde(rename = "SRP Registration Number")]
    pub application_id: String,
    #[serde(rename = "Premise Last Name")]
    pub last_name: Option<String>,
    #[serde(rename = "Premise Company")]
    pub company: Option<String>,
    #[serde(rename = "Premise Installation Address (Commercial Only)")]
    pub address: Option<String>,
    #[serde(rename = "Premise City")]
    pub city: Option<String>,
    #[serde(
        default,
        rename = "Premise                         Zip",
        deserialize_with = "crate::util::zip_code"
    )]
    pub zip_code: Option<String>,
    #[serde(rename = "County                      Code")]
    pub county_code: u8,
    #[serde(
        rename = "PTO Date (Interconnection Date)",
        deserialize_with = "crate::excel::date"
    )]
    pub pto_date: Option<NaiveDate>,
    #[serde(rename = "Calculated Total System Size")]
    pub system_size: f32,
    #[serde(
        rename = "Third Party Ownership",
        deserialize_with = "super::util::bool_from_string"
    )]
    pub third_party_ownership: bool,
    #[serde(rename = "Contractor Company")]
    pub installer: Option<String>,
    #[serde(rename = "Electric Utility Name")]
    pub utility: String,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct SRPInstallation(#[serde(with = "InstallationDef")] Installation);

impl Into<Installation> for SRPInstallation {
    fn into(self) -> Installation {
        self.0
    }
}
