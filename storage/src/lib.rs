use rusqlite::{Connection, Result, Statement};

mod database;

#[allow(unused_imports)]
#[macro_use]
extern crate storage_derive;
pub use storage_derive::*;

pub trait Store {
    fn initialize(conn: &mut Connection) -> Result<()>;

    fn upsert_statement(conn: &Connection) -> Result<Statement>;
    fn upsert(&self, statement: &mut Statement) -> Result<()>;

    const PROGRESS_NAME: &'static str;
}

pub use database::Database;

pub use chrono;
pub mod sqlite {
    pub use rusqlite::{Connection, Result, Statement, ToSql};
}
