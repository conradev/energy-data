use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar};
use std::thread;
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
    Assemble { dataset: Vec<String> },
}

// const index: (&'static str, impl Retrieve) = (("nj_solar", retrieve::nj_solar::Retriever))

async fn main_try() -> Result<()> {
    // let cli = Cli::parse();
    //
    // match cli.command {
    //     Commands::Fetch { name } => {
    //         println!("{:?}", name);
    //     }
    // }

    let multi_progress = MultiProgress::new();

    // let eia_progress = multi_progress.clone();
    // let eia: JoinHandle<Result<()>> = tokio::spawn(async move {
    //     let mut db = storage::Database::open("out.db")?;
    //     db.initialize::<schema::eia::Utility>()?;
    //     assemble_into::<retrieve::eia::Retriever>(&mut db, eia_progress).await?;
    //     Ok(())
    // });

    // let nj_progress = multi_progress.clone();
    // let nj: JoinHandle<Result<()>> = tokio::spawn(async move {
    //     let mut db = storage::Database::open("out.db")?;
    //     db.initialize::<schema::nj_solar::Installation>()?;
    //     assemble_into::<retrieve::nj_solar::Retriever>(&mut db, nj_progress).await?;
    //     Ok(())
    // });

    let ca: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut db = storage::Database::open("out.db")?;
        db.initialize::<schema::ca_solar::Installation>()?;
        assemble_into::<retrieve::ca_solar::Retriever>(&mut db, multi_progress).await?;
        Ok(())
    });

    // tokio::join!(ca, nj)?
    ca.await?

    // eia.await?
}

#[tokio::main]
async fn main() {
    main_try().await.unwrap()
}
