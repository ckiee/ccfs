use anyhow::Context;
use anyhow::{anyhow, Result};
use clap::Parser;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use include_dir::{include_dir, Dir};
use log::debug;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, io::Write, time::SystemTime};
use tokio::net::ToSocketAddrs;
use tokio::{
    join,
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum C2SMessage {
    Bootstrap,
    GetInternalFile { path: String },
    EvalResult { result: serde_json::Value },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum S2CMessage {
    Eval { code: String },
    PutFile { path: String, data: String },
}

static LUA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/lua");

fn get_file_string(path: &str) -> Result<String> {
    Ok(LUA_DIR
        .get_file(path)
        .ok_or(anyhow!("file not found"))?
        .contents_utf8()
        .ok_or(anyhow!("file not valid utf-8"))?
        .to_string())
}

fn get_file_bytes(path: &str) -> Result<&[u8]> {
    Ok(LUA_DIR
        .get_file(path)
        .ok_or(anyhow!("file not found"))?
        .contents())
}

pub async fn accept_connection(stream: TcpStream) -> Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // Rust is not ready for async non-move lambdas yet
    macro_rules! send_packet {
        ($packet: expr) => {{
            info!("[S2C ->] {:?}", &($packet));
            write
                .send(Message::Text(
                    serde_json::to_string(&($packet)).context("in send_packet macro")?,
                ))
                .await
                .context("in send_packet macro")?;
        }};
    }

    loop {
        let packet_json = match read.next().await {
            None => break,
            Some(m) => match m? {
                Message::Text(j) => j,
                catch => {
                    warn!("unimplemented websocket message {catch:?}");
                    continue;
                }
            },
        };
        debug!("[C2S <- <text>] {}", &packet_json);

        let packet = serde_json::from_str::<C2SMessage>(&packet_json)?;
        info!("[C2S <-] {packet:?}");

        match packet {
            C2SMessage::Bootstrap => send_packet!(S2CMessage::Eval {
                code: get_file_string("stage1.lua")?
            }),
            C2SMessage::GetInternalFile { path } => {
                send_packet!(S2CMessage::PutFile {
                    path: format!("/ccfs/{path}"),
                    // TODO: support binary files..
                    data: get_file_string(&path)?
                })
            }
            C2SMessage::EvalResult { result } => {
                info!("eval result: {result:?}"); //meoww
            }
        }
    }

    Ok(())
}
