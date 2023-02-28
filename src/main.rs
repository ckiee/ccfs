use anyhow::Context;
use anyhow::{anyhow, Result};
use clap::Parser;
use fuse_mt::FuseMT;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use include_dir::{include_dir, Dir};
use log::debug;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, io::Write, time::SystemTime};
use tokio::net::ToSocketAddrs;
use tokio::{
    join,
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::tungstenite::Message;

use crate::fs::CCFS;

/// FUSE filesystem for ComputerCraft
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    listen_address: String,
    #[arg(short, long)]
    mountpoint: PathBuf,
}

mod fs;
mod socket;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = env_logger::try_init();
    let args = Args::parse();

    let listener = TcpListener::bind(&args.listen_address).await?;
    info!("Listening on: {}", args.listen_address);

    let bg_fuse = fuse_mt::spawn_mount(
        FuseMT::new(CCFS::new(), 1),
        args.mountpoint,
        &(["auto_unmount", "allow_other"].map(OsStr::new))[..],
    )?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(socket::accept_connection(stream));
    }

    Ok(())
}
