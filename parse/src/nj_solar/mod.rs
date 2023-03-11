use anyhow::Result;
use calamine::Reader;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{Read, Seek};

mod adi;
mod srp;
mod ti;
mod util;


use crate::util::StreamLength;

pub struct Parser;

impl crate::ParseBuffer for Parser {
    type Record = schema::nj_solar::Installation;

    fn from_buffer<R: Read + Seek>(
        mut reader: R,
        progress: MultiProgress,
    ) -> Result<Vec<Self::Record>> {
        let file_len = StreamLength::stream_len(&mut reader)?;
        let load_progress = progress
            .add(ProgressBar::new(file_len))
            .with_style(
                ProgressStyle::with_template("{prefix}: {msg} {bar} {bytes}/{total_bytes}")
                    .unwrap(),
            )
            .with_prefix("nj_solar")
            .with_message("Loading Workbook");

        let mut workbook: calamine::Xlsx<_> =
            calamine::Reader::new(load_progress.wrap_read(reader))?;
        let contents: HashMap<_, _> = workbook.worksheets().into_iter().collect();

        load_progress.finish();

        let len = ["ADI - Installed", "TI - Installed", "SRP - Installed"]
            .into_iter()
            .filter_map(|s| contents.get(s))
            .map(|r| r.height() as u64)
            .sum();

        let parse_progress = progress
            .add(ProgressBar::new(len))
            .with_style(ProgressStyle::with_template("{prefix}: {msg} {bar} {pos}/{len}").unwrap())
            .with_prefix("nj_solar")
            .with_message("Parsing Sheets");

        let mut records: Vec<Self::Record> = vec![];

        if let Some(range) = contents.get("ADI - Installed") {
            let iter = calamine::RangeDeserializerBuilder::new()
                .from_range::<_, adi::ADIInstallation>(range)?
                .map(|r| r.map(|i| i.into()));
            let mut batch: Vec<_> = parse_progress.wrap_iter(iter).collect::<Result<_, _>>()?;
            records.append(&mut batch);
        }

        if let Some(range) = contents.get("TI - Installed") {
            let iter = calamine::RangeDeserializerBuilder::new()
                .from_range::<_, ti::TIInstallation>(range)?
                .map(|r| r.map(|i| i.into()));
            let mut batch: Vec<_> = parse_progress.wrap_iter(iter).collect::<Result<_, _>>()?;
            records.append(&mut batch);
        }

        if let Some(range) = contents.get("SRP - Installed") {
            let iter = calamine::RangeDeserializerBuilder::new()
                .from_range::<_, srp::SRPInstallation>(range)?
                .map(|r| r.map(|i| i.into()));
            let mut batch: Vec<_> = parse_progress.wrap_iter(iter).collect::<Result<_, _>>()?;
            records.append(&mut batch);
        }

        parse_progress.finish();

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use chrono::NaiveDate;
    use schema::nj_solar::Installation;

    #[test]
    fn parse_entire_dataset() {
        let records = parse_file!(Parser, "DATA+-+INSTALLED+-+SEPTEMBER+2022.xlsx");

        let adi_record = Installation {
            application_id: String::from("NJADRE1547570496"),
            last_name: Some(String::from("Ulrich")),
            company: None,
            address: None,
            city: Some(String::from("Delran")),
            zip_code: Some(String::from("08075")),
            county_code: 13,
            pto_date: Some(NaiveDate::from_ymd(2021, 12, 20)),
            system_size: 12.08,
            third_party_ownership: false,
            installer: Some(String::from("Suntuity Solar LLC")),
            utility: String::from("PSE&G"),
        };
        let ti_record = Installation {
            application_id: String::from("NJSRRE1532376477"),
            last_name: Some(String::from("FRIEDMAN")),
            company: None,
            address: None,
            city: Some(String::from("CHERRY HILL")),
            zip_code: Some(String::from("08003")),
            county_code: 14,
            pto_date: Some(NaiveDate::from_ymd(2010, 2, 15)),
            system_size: 8.74,
            third_party_ownership: false,
            installer: Some(String::from("RENERGY, INC.")),
            utility: String::from("PSE&G BPU"),
        };

        for record in vec![adi_record, ti_record] {
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
}
