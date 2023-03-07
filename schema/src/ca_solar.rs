use chrono::NaiveDate;
use storage::Store;

#[derive(Debug, Clone, PartialEq, Store)]
#[storage(table = "ca_solar_installation", primary_key = "application_id")]
pub struct Installation {
    pub application_id: String,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub county: Option<String>,
    pub recieved_date: Option<NaiveDate>,
    pub complete_date: Option<NaiveDate>,
    pub approved_date: Option<NaiveDate>,
    pub system_size_dc: f32,
    pub system_size_ac: f32,
    pub installer: Option<String>,
    pub utility: String,
}
