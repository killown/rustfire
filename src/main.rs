use std::io;
mod ipc;
mod models;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = ipc::WayfireSocket::connect().await?;
    let views = socket.list_views().await?;
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;

    // Print Views in JSON format
    for view in &views {
        let view_json = serde_json::to_string_pretty(&view)?;
        println!("View JSON: {}", view_json);
    }

    // Print Outputs in JSON format
    for output in &outputs {
        let output_json = serde_json::to_string_pretty(&output)?;
        println!("Output JSON: {}", output_json);
    }

    // Print Wsets in JSON format
    for wset in &wsets {
        let wset_json = serde_json::to_string_pretty(&wset)?;
        println!("Wset JSON: {}", wset_json);
    }

    let input_devices = socket.list_input_devices().await?;
    let input_devices_json = serde_json::to_string_pretty(&input_devices)?;
    println!("Input devices JSON: {}", input_devices_json);

    match socket.get_configuration().await {
        Ok(config) => {
            let config_json = serde_json::to_string_pretty(&config)?;
            println!("Wayfire Configuration JSON: {}", config_json);
        }
        Err(e) => {
            eprintln!("Failed to get configuration: {}", e);
        }
    }

    match socket.get_option_value("core/plugins").await {
        Ok(response) => {
            let response_json = serde_json::to_string_pretty(&response)?;
            println!("Option Value Response JSON: {}", response_json);
        }
        Err(e) => {
            eprintln!("Failed to get option value: {}", e);
        }
    }

    match socket.get_output(1).await {
        Ok(output) => {
            let output_json = serde_json::to_string_pretty(&output)?;
            println!("Output JSON: {}", output_json);
        }
        Err(e) => eprintln!("Failed to get output: {:?}", e),
    }

    // Access the ID of the first view (if it exists)
    if let Some(view) = views.get(0) {
        let view_id = view.id;
        let detailed_view = socket.get_view(view_id).await?;
        let detailed_view_json = serde_json::to_string_pretty(&detailed_view)?;
        println!("Detailed View JSON: {}", detailed_view_json);
    } else {
        println!("No views found.");
    }

    Ok(())
}
