use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar};
use std::io::{Read, Seek};

pub trait ParseBuffer {
    type Record: Send;

    fn from_buffer<R: Read + Seek>(reader: R, progress: MultiProgress)
        -> Result<Vec<Self::Record>>;
}

#[cfg(test)]
macro_rules! test_file {
    ($path:tt) => {{
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push(file!());
        path.pop();
        path.push($path);
        let file = std::fs::File::open(path).unwrap();
        file
    }};
}

#[cfg(test)]
macro_rules! parse_file {
    ($parser:ty, $file:tt) => {{
        use crate::ParseBuffer;

        let file = test_file!($file);
        let records = <$parser>::from_buffer(file, indicatif::MultiProgress::new()).unwrap();
        records
    }};
}

pub(crate) mod excel;
pub(crate) mod util;
pub(crate) mod zip;

pub mod ca_solar;
pub mod eia;
pub mod lbnl_solar;
pub mod nj_solar;
