use std::io;
use serde_json::to_string_pretty;

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
        print_json("View", view).await?;
    }

    for output in &outputs {
        print_json("Output", output).await?;
    }

    for wset in &wsets {
        print_json("Wset", wset).await?;
    }

    print_json("Input devices", input_devices).await?;

    match socket.get_configuration().await {
        Ok(config) => print_json("Wayfire Configuration", config).await?,
        Err(e) => eprintln!("Failed to get configuration: {}", e),
    }

    match socket.get_option_value("core/plugins").await {
        Ok(response) => print_json("Option Value Response", response).await?,
        Err(e) => eprintln!("Failed to get option value: {}", e),
    }

    match socket.get_output(1).await {
        Ok(output) => print_json("Output", output).await?,
        Err(e) => eprintln!("Failed to get output: {:?}", e),
    }

    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view(view_id).await {
            Ok(detailed_view) => print_json("Detailed View", detailed_view).await?,
            Err(e) => eprintln!("Failed to get detailed view: {}", e),
        }
    } else {
        println!("No views found.");
    }

    Ok(())
}

