use anyhow::Result;
use glob::Pattern;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::{Cursor, Read, Seek};
use std::ops::Mul;

use crate::util::StreamLength;

mod mapping;

pub type ArchiveParser = crate::zip::ZipBufferParser<Parser, Parser>;

pub struct Parser;

impl crate::zip::ParseZipArchive for Parser {
    const PROGRESS_NAME: &'static str = "ca_solar";

    fn filter() -> Pattern {
        Pattern::new("*.csv").unwrap()
    }
}

impl crate::ParseBuffer for Parser {
    type Record = schema::ca_solar::Installation;

    fn from_buffer<R: Read + Seek>(
        mut reader: R,
        progress: MultiProgress,
    ) -> Result<Vec<Self::Record>> {
        let len = StreamLength::stream_len(&mut reader)?;
        let progress = progress
            .add(ProgressBar::new(len))
            .with_style(
                ProgressStyle::with_template("{prefix}: {msg} {bar} {bytes}/{total_bytes}")
                    .unwrap(),
            )
            .with_prefix("ca_solar")
            .with_message("Parsing CSV");

        let mut reader = csv::Reader::from_reader(progress.wrap_read(reader));
        let records: Vec<_> = reader
            .deserialize::<mapping::Mapping>()
            .map(|r| r.map(|v| v.into()))
            .filter_map(|r| r.ok()) // TODO: Log
            .collect();

        progress.finish();

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use chrono::NaiveDate;
    use schema::ca_solar::Installation;

    #[test]
    fn parse_sce_dataset() {
        let records = parse_file!(Parser, "SCE_Interconnected_Project_Sites_2022-08-30.csv");

        let record = Installation {
            application_id: String::from("SCE-INT-500005417"),
            city: Some(String::from("LAGUNA BEACH")),
            zip_code: Some(String::from("92651")),
            county: Some(String::from("Orange")),
            recieved_date: NaiveDate::from_ymd_opt(2011, 3, 8),
            complete_date: None,
            approved_date: NaiveDate::from_ymd_opt(2011, 7, 25),
            system_size_dc: 4.081632653,
            system_size_ac: 4.0,
            installer: Some(String::from("REC SOLAR")),
            utility: String::from("SCE"),
        };
        assert_eq!(
            record,
            records
                .iter()
                .find(|r| record.application_id == r.application_id)
                .cloned()
                .unwrap()
        );
    }
}
