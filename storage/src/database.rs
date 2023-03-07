use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rusqlite::{Connection, DatabaseName, Result};
use std::path::Path;

use crate::Store;

pub struct Database(Connection);

impl Database {
    pub fn new() -> Result<Database> {
        Ok(Database(Connection::open_in_memory()?))
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
        Ok(Database(Connection::open(path)?))
    }

    pub fn backup<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.0.backup(DatabaseName::Main, path, None)
    }

    pub fn initialize<R: Store>(&mut self) -> Result<()> {
        R::initialize(&mut self.0)
    }

    pub fn insert<R>(&mut self, records: Vec<R>, progress: MultiProgress) -> Result<()>
    where
        R: Store,
    {
        let progress = progress
            .add(ProgressBar::new(records.len() as u64))
            .with_style(ProgressStyle::with_template("{prefix}: {msg} {bar} {pos}/{len}").unwrap())
            .with_prefix(R::PROGRESS_NAME)
            .with_message("Inserting");

        let tx = self.0.transaction()?;
        {
            let mut statement = R::upsert_statement(&tx)?;
            for record in progress.wrap_iter(records.into_iter()) {
                record.upsert(&mut statement)?;
            }
        }
        tx.commit()?;
        progress.finish();

        Ok(())
    }
}
