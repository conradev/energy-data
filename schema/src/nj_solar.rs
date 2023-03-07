use chrono::NaiveDate;
use storage::Store;

#[derive(Debug, Clone, PartialEq, Store)]
#[storage(table = "nj_solar_installation", primary_key = "application_id")]
pub struct Installation {
    pub application_id: String,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub county_code: u8,
    pub pto_date: Option<NaiveDate>,
    pub system_size: f32,
    pub third_party_ownership: bool,
    pub installer: Option<String>,
    pub utility: String,
}
