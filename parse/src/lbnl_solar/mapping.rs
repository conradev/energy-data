use chrono::NaiveDate;
use serde::Deserialize;

use schema::lbnl_solar::Installation;

#[derive(Deserialize)]
#[serde(remote = "Installation")]
pub struct InstallationDef {
    pub data_provider_1: String,
    pub data_provider_2: String,
    #[serde(rename = "system_ID_1")]
    pub system_id_1: String,
    #[serde(rename = "system_ID_2")]
    pub system_id_2: String,
    pub installation_date: NaiveDate,
    pub system_size_dc: f32,
    pub total_installed_price: f32,
    pub rebate_or_grant: f32,
    pub customer_segment: String,
    pub zip_code: String,
    pub city: String,
    pub state: String,
    pub utility_service_territory: String,
    pub installer_name_1: String,
    pub module_manufacturer_1: String,
    pub module_model_1: String,
    pub module_quantity_1: u32,
    pub inverter_manufacturer_1: String,
    pub inverter_model_1: String,
    pub inverter_quantity_1: u32,
    pub output_capacity_inverter_1: f32,
    pub battery_manufacturer: String,
    pub battery_model: String,
    #[serde(rename = "battery_rated_capacity_kW")]
    pub battery_rated_capacity_kw: String,
    #[serde(rename = "battery_rated_capacity_kWh")]
    pub battery_rated_capacity_kwh: String,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Mapping(#[serde(with = "InstallationDef")] Installation);

impl Into<Installation> for Mapping {
    fn into(self) -> Installation {
        self.0
    }
}
