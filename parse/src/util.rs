use serde::{Deserialize, Deserializer};
use std::io::{Seek, SeekFrom};

pub fn zip_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| format!("{:0>5}", s)))
}

pub trait StreamLength: Seek {
    fn stream_len(&mut self) -> Result<u64, std::io::Error> {
        let old_pos = self.stream_position()?;
        let len = self.seek(SeekFrom::End(0))?;

        if old_pos != len {
            self.seek(SeekFrom::Start(old_pos))?;
        }

        Ok(len)
    }
}

impl<T> StreamLength for T where T: Seek {}

macro_rules! bool_from_string {
    ($affirmative:literal, $negative:literal) => {
        pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::Deserialize;

            match String::deserialize(deserializer)?.as_ref() {
                $affirmative => Ok(true),
                $negative => Ok(false),
                other => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(other),
                    &concat!($affirmative, " or ", $negative),
                )),
            }
        }
    };
}

// macro_rules! date_from_string {
//     ($format:literal) => {
//         pub fn date_from_string<'de, D>(
//             deserializer: D,
//         ) -> Result<Option<chrono::NaiveDate>, D::Error>
//         where
//             D: serde::Deserializer<'de>,
//         {
//             use serde::Deserialize;
//
//             let string = String::deserialize(deserializer)?;
//             if string.is_empty() {
//                 return Ok(None);
//             }
//             match chrono::NaiveDate::parse_from_str(&string, $format) {
//                 Ok(d) => Ok(Some(d)),
//                 Err(_) => Err(serde::de::Error::invalid_value(
//                     serde::de::Unexpected::Str(&string),
//                     &concat!("Date in the format ", $format),
//                 )),
//             }
//         }
//     };
// }

pub(crate) use bool_from_string;
// pub(crate) use date_from_string;
