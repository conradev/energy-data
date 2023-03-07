use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar};
use retrieve::Retrieve;
use std::ops::Mul;
use storage::{Database, Store};

pub async fn assemble_into<R>(db: &mut storage::Database, progress: MultiProgress) -> Result<()>
where
    R: Retrieve,
    R::Record: Store,
{
    let records: Vec<R::Record> = R::retrieve(progress.clone()).await?;
    db.insert(records, progress)?;
    Ok(())
}
