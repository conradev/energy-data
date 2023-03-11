use schema::eia::Utility;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(remote = "Utility")]
pub struct UtilityDef {
    #[serde(rename = "Utility Number")]
    pub number: u32,
    #[serde(rename = "Utility Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Ownership Type")]
    pub ownership: String,
    #[serde(rename = "NERC Region")]
    pub nerc_region: String,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct ParsedUtility(#[serde(with = "UtilityDef")] Utility);

impl From<ParsedUtility> for Utility {
    fn from(val: ParsedUtility) -> Self {
        val.0
    }
}
