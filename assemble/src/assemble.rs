use anyhow::Result;
use indicatif::{MultiProgress};
use retrieve::Retrieve;

use storage::{Store};

pub async fn assemble_into<R>(db: &mut storage::Database, progress: MultiProgress) -> Result<()>
where
    R: Retrieve,
    R::Record: Store,
{
    let records: Vec<R::Record> = R::retrieve(progress.clone()).await?;
    db.insert(records, progress)?;
    Ok(())
}
