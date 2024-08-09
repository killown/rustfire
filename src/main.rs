use serde_json::to_string_pretty;
use std::io;

mod ipc;
mod models;

async fn print_json<T: serde::Serialize>(label: &str, data: T) -> io::Result<()> {
    let json = to_string_pretty(&data)?;
    println!("{} JSON: {}", label, json);
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = ipc::WayfireSocket::connect().await?;

    let views = socket.list_views().await?;
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;
    let input_devices = socket.list_input_devices().await?;

    for view in &views {
        print_json("list_views", view).await?;
    }

    for output in &outputs {
        print_json("list_outputs", output).await?;
    }

    for wset in &wsets {
        print_json("list_wsets", wset).await?;
    }

    print_json("list_input_devices", input_devices).await?;

    match socket.get_configuration().await {
        Ok(config) => print_json("get_configuration", config).await?,
        Err(e) => eprintln!("Failed to get configuration: {}", e),
    }

    match socket.get_option_value("core/plugins").await {
        Ok(response) => print_json("get_option_value", response).await?,
        Err(e) => eprintln!("Failed to get option value: {}", e),
    }

    match socket.get_output(1).await {
        Ok(output) => print_json("get_output", output).await?,
        Err(e) => eprintln!("Failed to get output: {:?}", e),
    }

    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view(view_id).await {
            Ok(detailed_view) => print_json("get_view:", detailed_view).await?,
            Err(e) => eprintln!("Failed to get detailed view: {}", e),
        }
    } else {
        println!("No views found.");
    }

    match socket.get_focused_view().await {
        Ok(view) => print_json("get_focused_view:", view).await?,
        Err(e) => eprintln!("Failed to get focused view: {}", e),
    }

    Ok(())
}
