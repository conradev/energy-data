use storage::Store;

#[derive(Debug, Clone, PartialEq, Store)]
#[storage(table = "eia_utility", primary_key = "number")]
pub struct Utility {
    pub number: u32,
    pub name: String,
    pub state: String,
    pub ownership: String,
    pub nerc_region: String,
}
