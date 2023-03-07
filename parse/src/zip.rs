use crate::nj_solar::Parser;
use crate::ParseBuffer;
use anyhow::Result;
use glob::Pattern;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek};
use std::marker::PhantomData;

pub trait ParseZipArchive {
    const PROGRESS_NAME: &'static str = "";

    fn filter() -> Pattern;
}

pub struct ZipBufferParser<P: ParseBuffer, Z: ParseZipArchive> {
    a: PhantomData<P>,
    z: PhantomData<Z>,
}

impl<P, Z> ParseBuffer for ZipBufferParser<P, Z>
where
    P: ParseBuffer,
    Z: ParseZipArchive,
{
    type Record = P::Record;

    fn from_buffer<R: Read + Seek>(
        reader: R,
        progress: MultiProgress,
    ) -> Result<Vec<Self::Record>> {
        let mut zip = zip::ZipArchive::new(reader)?;
        let mut records = vec![];
        for idx in 0..zip.len() {
            let mut file = zip.by_index(idx)?;

            let Some(path) = file.enclosed_name() else { continue };
            if !Z::filter().matches_path(path) {
                continue;
            }

            let len = file.size();

            let inflate_progress = progress
                .add(ProgressBar::new(len))
                .with_style(
                    ProgressStyle::with_template("{prefix}: {msg} {bar} {bytes}/{total_bytes}")
                        .unwrap(),
                )
                .with_prefix(Z::PROGRESS_NAME)
                .with_message("Inflating CSV");

            let mut buf = Vec::with_capacity(len as usize);

            inflate_progress.wrap_read(file).read_to_end(&mut buf)?;
            inflate_progress.finish();

            let reader = Cursor::new(buf);

            let mut batch = P::from_buffer(reader, progress.clone())?;
            records.append(&mut batch);
        }

        Ok(records)
    }
}
