use anyhow::Result;
use glob::Pattern;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::{Read, Seek};


use crate::util::StreamLength;

mod mapping;

pub type ArchiveParser = crate::zip::ZipBufferParser<Parser, Parser>;

pub struct Parser;

impl crate::zip::ParseZipArchive for Parser {
    const PROGRESS_NAME: &'static str = "lbnl_solar";

    fn filter() -> Pattern {
        Pattern::new("*.csv").unwrap()
    }
}

impl crate::ParseBuffer for Parser {
    type Record = schema::lbnl_solar::Installation;

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
        let records: Result<Vec<_>, _> = reader
            .deserialize::<mapping::Mapping>()
            .map(|r| r.map(|v| v.into()))
            // .filter_map(|r| r.ok()) // TODO: Log
            .collect();

        progress.finish();

        Ok(records?)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use chrono::NaiveDate;
    use schema::lbnl_solar::Installation;

    #[test]
    fn parse_lbnl_dataset() {
        let records = parse_file!(Parser, "out.csv");
        println!("{records:?}");

        let record = Installation {
            data_provider_1: String::from("Massachusetts Clean Energy Center"),
            data_provider_2: String::from("Massachusetts Department of Energy Resources"),
            system_id_1: String::from("CPV-MA19-00010"),
            system_id_2: String::from("-1"),
            installation_date: NaiveDate::from_ymd_opt(2003, 05, 09).unwrap(),
            system_size_dc: 2.24,
            total_installed_price: 18616.53,
            rebate_or_grant: 8568.0,
            customer_segment: String::from("RES"),
            zip_code: String::from("02568"),
            city: String::from("Tisbury"),
            state: String::from("MA"),
            utility_service_territory: String::from("NSTAR DBA EverSource"),
            installer_name_1: String::from("Mv Electricians"),
            module_manufacturer_1: String::from("-1"),
            module_model_1: String::from("-1"),
            module_quantity_1: 8,
            inverter_manufacturer_1: String::from("SMA America"),
            inverter_model_1: String::from("SWR 2100U"),
            inverter_quantity_1: 1,
            output_capacity_inverter_1: 2.1,
            battery_manufacturer: String::from("-1"),
            battery_model: String::from("-1"),
            battery_rated_capacity_kw: String::from("-1"),
            battery_rated_capacity_kwh: String::from("-1"),
        };
        assert_eq!(
            record,
            records
                .iter()
                .find(|r| record.system_id_1 == r.system_id_1)
                .cloned()
                .unwrap()
        );
    }
}
