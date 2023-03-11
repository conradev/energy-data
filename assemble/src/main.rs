use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use std::path::PathBuf;
use tokio::task::JoinHandle;

mod assemble;

use assemble::assemble_into;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Assemble { output: PathBuf },
}

async fn main_try() -> Result<()> {
    let cli = Cli::parse();
    let output_path = match cli.command {
        Commands::Assemble { output } => output,
    };

    let multi_progress = MultiProgress::new();
    let path = "file:energy?mode=memory&cache=shared";
    let db = storage::Database::open(path)?;

    let eia_progress = multi_progress.clone();
    let eia: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut db = storage::Database::open(path)?;
        db.initialize::<schema::eia::Utility>()?;
        assemble_into::<retrieve::eia::Retriever>(&mut db, eia_progress).await?;
        Ok(())
    });

    let nj_progress = multi_progress.clone();
    let nj: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut db = storage::Database::open(path)?;
        db.initialize::<schema::nj_solar::Installation>()?;
        assemble_into::<retrieve::nj_solar::Retriever>(&mut db, nj_progress).await?;
        Ok(())
    });

    let ca: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut db = storage::Database::open(path)?;
        db.initialize::<schema::ca_solar::Installation>()?;
        assemble_into::<retrieve::ca_solar::Retriever>(&mut db, multi_progress).await?;
        Ok(())
    });

    let (eia_res, ca_res, nj_res) = tokio::join!(eia, ca, nj);
    eia_res??;
    ca_res??;
    nj_res??;

    db.vacuum_into(output_path)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    main_try().await.unwrap()
}
