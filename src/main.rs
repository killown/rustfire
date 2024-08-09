use std::io;
mod ipc;
mod models;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = ipc::WayfireSocket::connect().await?;
    let views = socket.list_views().await?;
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;

    // Iterate over references to avoid moving `views`
    for view in &views {
        println!("{:?}", view);
    }

    for output in &outputs {
        println!("{:?}", output);
    }

    for wset in &wsets {
        println!("{:?}", wset);
    }

    let input_devices = socket.list_input_devices().await?;
    println!("Input devices: {:?}", input_devices);

    match socket.get_configuration().await {
        Ok(config) => {
            println!("Wayfire Configuration: {:?}", config);
        }
        Err(e) => {
            eprintln!("Failed to get configuration: {}", e);
        }
    }

    match socket.get_option_value("core/plugins").await {
        Ok(response) => {
            println!("Option Value Response: {:?}", response);
        }
        Err(e) => {
            eprintln!("Failed to get option value: {}", e);
        }
    }

    match socket.get_output(1).await {
        Ok(output) => println!("{:?}", output),
        Err(e) => eprintln!("Failed to get output: {:?}", e),
    }

    // Access the ID of the first view (if it exists)
    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view(view_id).await {
            Ok(detailed_view) => println!("{:?}", detailed_view),
            Err(e) => eprintln!("Failed to get view: {:?}", e),
        }
    } else {
        println!("No views found.");
    }

    Ok(())
}

