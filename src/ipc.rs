use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream as TokioUnixStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgTemplate {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct View {
    pub id: i32,
    pub role: String,
    pub mapped: bool,
    pub pid: i32,
}

pub struct WayfireSocket {
    client: TokioUnixStream,
}

impl WayfireSocket {
    pub async fn connect() -> io::Result<Self> {
        let socket_name = env::var("WAYFIRE_SOCKET")
            .expect("WAYFIRE_SOCKET environment variable not set");
        let client = TokioUnixStream::connect(&socket_name).await?;
        Ok(WayfireSocket { client })
    }

    pub async fn send_json(&mut self, msg: &MsgTemplate) -> io::Result<serde_json::Value> {
        let data = serde_json::to_vec(msg)?;
        let header = (data.len() as u32).to_le_bytes();

        self.client.write_all(&header).await?;
        self.client.write_all(&data).await?;

        self.read_message().await
    }

    pub async fn read_exact(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.client.read_exact(&mut buf).await?;
        Ok(buf)
    }

    pub async fn read_message(&mut self) -> io::Result<serde_json::Value> {
        let len_buf = self.read_exact(4).await?;
        let len = u32::from_le_bytes(len_buf.try_into().unwrap()) as usize;

        let response_buf = self.read_exact(len).await?;
        let response: serde_json::Value = serde_json::from_slice(&response_buf)?;

        if response.get("error").is_some() {
            eprintln!("Error: {:?}", response);
        }

        Ok(response)
    }

    pub async fn list_views(&mut self, filter_mapped_toplevel: bool) -> io::Result<Vec<View>> {
        let message = MsgTemplate {
            method: "window-rules/list-views".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let views: Vec<View> = serde_json::from_value(response)?;

        let filtered_views = if filter_mapped_toplevel {
            views
                .into_iter()
                .filter(|view| view.mapped && view.role == "toplevel")
                .collect()
        } else {
            views
        };

        Ok(filtered_views)
    }
}

