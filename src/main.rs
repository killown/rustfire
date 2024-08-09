use std::io;
mod ipc;
mod models;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = ipc::WayfireSocket::connect().await?;
    let views = socket.list_views().await?;
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;
    for view in views {
        println!("{:?}", view);
    }
    for output in outputs {
        println!("{:?}", output);
    }

    for wset in wsets {
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

    Ok(())
}
