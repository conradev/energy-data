use anyhow::{anyhow, Result};
use calamine::DataType;
use chrono::NaiveDate;
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::io::{Read, Seek};

pub trait ParseSheet {
    fn parse<T, U>(&mut self, name: &str, progress: Option<ProgressBar>) -> Result<Vec<U>>
    where
        T: DeserializeOwned,
        T: Into<U>;
}

impl<RS> ParseSheet for calamine::Xlsx<RS>
where
    RS: Read + Seek,
{
    fn parse<T, U>(&mut self, name: &str, progress: Option<ProgressBar>) -> Result<Vec<U>>
    where
        T: DeserializeOwned,
        T: Into<U>,
    {
        use calamine::Reader;

        let progress = progress.unwrap_or_else(|| ProgressBar::hidden());
        progress.set_style(ProgressStyle::with_template("{prefix}: {msg} {spinner}").unwrap());
        progress.set_message("Scanning Workbook");

        let range = self
            .worksheet_range(name)
            .ok_or(anyhow!("Could not find sheet \"{}\"", name))??;

        progress.set_length(range.height() as u64);
        progress
            .set_style(ProgressStyle::with_template("{prefix}: {msg} {bar} {pos}/{len}").unwrap());
        progress.set_message("Parsing Workbook");

        let iter = progress.wrap_iter(
            calamine::RangeDeserializerBuilder::new()
                .from_range::<_, T>(&range)?
                .map(|r| r.map(|v| v.into())),
        );

        let records: Result<Vec<U>, _> = iter.collect();
        progress.finish();

        Ok(records?)
    }
}

pub fn date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Result<DataType, _> = Deserialize::deserialize(deserializer);
    let value = match result {
        Ok(DataType::DateTime(v)) => v,
        Ok(DataType::Float(v)) => v,
        Ok(v) => {
            return Err(serde::de::Error::custom(format!(
                "Invalid Excel data type for date {:?}",
                v
            )))
        }
        Err(e) => return Err(e),
    };
    Ok(NaiveDate::from_ymd_opt(1899, 12, 30).map(|d| d + chrono::Duration::days(value as i64)))
}
