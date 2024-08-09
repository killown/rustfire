use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream as TokioUnixStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgTemplate {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputDevice {
    pub id: i64, // Changed from String to i64
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionValueResponse {
    pub default: String,
    pub result: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WayfireConfiguration {
    #[serde(rename = "api-version")]
    pub api_version: u32,
    #[serde(rename = "build-branch")]
    pub build_branch: String,
    #[serde(rename = "build-commit")]
    pub build_commit: String,
    #[serde(rename = "plugin-path")]
    pub plugin_path: String,
    #[serde(rename = "plugin-xml-dir")]
    pub plugin_xml_dir: String,
    #[serde(rename = "xwayland-support")]
    pub xwayland_support: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct View {
    pub activated: bool,
    #[serde(rename = "app-id")]
    pub app_id: String,
    #[serde(rename = "base-geometry")]
    pub base_geometry: Geometry,
    pub bbox: Geometry,
    pub focusable: bool,
    pub fullscreen: bool,
    pub geometry: Geometry,
    pub id: i64,
    #[serde(rename = "last-focus-timestamp")]
    pub last_focus_timestamp: i64,
    pub layer: String,
    pub mapped: bool,
    #[serde(rename = "max-size")]
    pub max_size: Size,
    #[serde(rename = "min-size")]
    pub min_size: Size,
    pub minimized: bool,
    #[serde(rename = "output-id")]
    pub output_id: i64,
    #[serde(rename = "output-name")]
    pub output_name: String,
    pub parent: i64,
    pub pid: i64,
    pub role: String,
    pub sticky: bool,
    #[serde(rename = "tiled-edges")]
    pub tiled_edges: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "wset-index")]
    pub wset_index: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub height: u64,
    pub width: u64,
    pub x: u64,
    pub y: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub height: u64,
    pub width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub geometry: Geometry,
    pub id: i64,
    pub name: String,
    #[serde(rename = "workarea")]
    pub work_area: Geometry,
    #[serde(rename = "workspace")]
    pub workspace: Workspace,
    #[serde(rename = "wset-index")]
    pub wset_index: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    #[serde(rename = "grid_height")]
    pub grid_height: u64,
    #[serde(rename = "grid_width")]
    pub grid_width: u64,
    pub x: u64,
    pub y: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSet {
    #[serde(rename = "index")]
    pub index: u64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "output-id")]
    pub output_id: i64,
    #[serde(rename = "output-name")]
    pub output_name: String,
    #[serde(rename = "workspace")]
    pub workspace: Workspace,
}

pub struct WayfireSocket {
    client: TokioUnixStream,
}

impl WayfireSocket {
    pub async fn connect() -> io::Result<Self> {
        let socket_name =
            env::var("WAYFIRE_SOCKET").expect("WAYFIRE_SOCKET environment variable not set");
        let client = TokioUnixStream::connect(&socket_name).await?;
        Ok(WayfireSocket { client })
    }

    pub async fn send_json(&mut self, msg: &MsgTemplate) -> io::Result<Value> {
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

    pub async fn read_message(&mut self) -> io::Result<Value> {
        let len_buf = self.read_exact(4).await?;
        let len = u32::from_le_bytes(len_buf.try_into().unwrap()) as usize;

        let response_buf = self.read_exact(len).await?;
        let response: Value = serde_json::from_slice(&response_buf)?;

        if response.get("error").is_some() {
            eprintln!("Error: {:?}", response);
        }

        Ok(response)
    }

    pub async fn list_views(&mut self) -> io::Result<Vec<View>> {
        let message = MsgTemplate {
            method: "window-rules/list-views".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let views: Vec<View> = serde_json::from_value(response)?;

        Ok(views)
    }

    pub async fn list_outputs(&mut self) -> io::Result<Vec<Output>> {
        let message = MsgTemplate {
            method: "window-rules/list-outputs".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let outputs: Vec<Output> = serde_json::from_value(response)?;

        Ok(outputs)
    }

    pub async fn list_wsets(&mut self) -> io::Result<Vec<WorkspaceSet>> {
        let message = MsgTemplate {
            method: "window-rules/list-wsets".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let workspace_sets: Vec<WorkspaceSet> = serde_json::from_value(response)?;

        Ok(workspace_sets)
    }

    pub async fn list_input_devices(&mut self) -> io::Result<Vec<InputDevice>> {
        let message = MsgTemplate {
            method: "input/list-devices".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let input_devices: Vec<InputDevice> = serde_json::from_value(response)?;

        Ok(input_devices)
    }

    pub async fn get_configuration(&mut self) -> io::Result<WayfireConfiguration> {
        let message = MsgTemplate {
            method: "wayfire/configuration".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let configuration: WayfireConfiguration = serde_json::from_value(response)?;

        Ok(configuration)
    }
    pub async fn get_option_value(&mut self, option: &str) -> io::Result<OptionValueResponse> {
        let message = MsgTemplate {
            method: "wayfire/get-config-option".to_string(),
            data: Some(serde_json::json!({
                "option": option
            })),
        };

        let response = self.send_json(&message).await?;
        let option_value_response: OptionValueResponse = serde_json::from_value(response)?;

        Ok(option_value_response)
    }
}
