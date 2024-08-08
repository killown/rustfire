use std::io;
mod ipc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut wayfire_socket = ipc::WayfireSocket::connect().await?;
    let views = wayfire_socket.list_views().await?;
    let outputs = wayfire_socket.list_outputs().await?;
    for view in views {
        println!("{:?}", view);
    }
    for output in outputs {
        println!("{:?}", output);
    }

    Ok(())
}
