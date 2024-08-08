mod ipc;
use ipc::WayfireSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut wayfire_socket = WayfireSocket::connect().await?;
    let views = wayfire_socket.list_views(true).await?;
    for view in views {
        println!("{:?}", view);
    }
    Ok(())
}
