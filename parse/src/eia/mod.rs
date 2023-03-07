use crate::eia::mapping::ParsedUtility;
use anyhow::Result;
use calamine::Reader;
use glob::{glob, Pattern};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::excel::ParseSheet;
use crate::util::StreamLength;

mod mapping;

pub type ArchiveParser = crate::zip::ZipBufferParser<Parser, Parser>;

pub struct Parser;

impl crate::zip::ParseZipArchive for Parser {
    const PROGRESS_NAME: &'static str = "eia";

    fn filter() -> Pattern {
        Pattern::new("Utility_Data_2021.xlsx").unwrap()
    }
}

impl crate::ParseBuffer for Parser {
    type Record = schema::eia::Utility;

    fn from_buffer<R: Read + Seek>(
        mut reader: R,
        progress: MultiProgress,
    ) -> Result<Vec<Self::Record>> {
        let len = StreamLength::stream_len(&mut reader)?;
        let contents = {
            let load_progress = progress
                .add(ProgressBar::new(len))
                .with_style(
                    ProgressStyle::with_template("{prefix}: {msg} {bar} {bytes}/{total_bytes}")
                        .unwrap(),
                )
                .with_prefix("eia")
                .with_message("Loading Workbook");

            let mut workbook: calamine::Xlsx<_> =
                calamine::Reader::new(load_progress.wrap_read(reader))?;
            let contents: HashMap<_, _> = workbook.worksheets().into_iter().collect();

            load_progress.finish();
            contents
        };

        let len = contents.values().map(|r| (r.height() - 2) as u64).sum();
        let records = {
            let parse_progress = progress
                .add(ProgressBar::new(len))
                .with_style(
                    ProgressStyle::with_template("{prefix}: {msg} {bar} {pos}/{len}").unwrap(),
                )
                .with_prefix("eia")
                .with_message("Parsing Sheets");

            let mut records: Vec<Self::Record> = vec![];

            for (_, table) in contents.into_iter() {
                let data = table.range((1, 0), table.end().unwrap());
                let iter = calamine::RangeDeserializerBuilder::new()
                    .from_range::<_, ParsedUtility>(&data)?
                    .map(|r| r.map(|i| i.into()));
                let mut batch: Vec<_> = parse_progress.wrap_iter(iter).collect::<Result<_, _>>()?;
                records.append(&mut batch);
            }

            parse_progress.finish();
            records
        };

        Ok(records)
    }
}
