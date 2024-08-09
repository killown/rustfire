use crate::models::{
    InputDevice, MsgTemplate, OptionValueResponse, Output, View, WayfireConfiguration, WorkspaceSet,
};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream as TokioUnixStream;

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

    pub async fn get_output(&mut self, output_id: i64) -> io::Result<Output> {
        let message = MsgTemplate {
            method: "window-rules/output-info".to_string(),
            data: Some(serde_json::json!({
                "id": output_id
            })),
        };

        let response = self.send_json(&message).await?;
        let output: Output = serde_json::from_value(response)?;

        Ok(output)
    }

    pub async fn get_view(&mut self, view_id: i64) -> io::Result<View> {
        let message = MsgTemplate {
            method: "window-rules/view-info".to_string(),
            data: Some(serde_json::json!({
                "id": view_id
            })),
        };

        let response = self.send_json(&message).await?;

        let info = response.get("info").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Missing 'info' field in response")
        })?;

        let view: View = serde_json::from_value(info.clone())?;

        Ok(view)
    }

    pub async fn get_focused_view(&mut self) -> Result<View, Box<dyn Error>> {
        let message = MsgTemplate {
            method: "window-rules/get-focused-view".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;

        let view_info = response.get("info").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Missing 'info' field in response")
        })?;

        let view: View = serde_json::from_value(view_info.clone())?;

        Ok(view)
    }
}
