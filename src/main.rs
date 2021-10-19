use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("10.33.0.17:6142").await?;
    let server_address = listener.local_addr().expect("unable to get the server address");
    println!("server ip is {}, listening on port {}", server_address.ip(), server_address.port());
    loop {
        let (mut socket, _) = listener.accept().await?;
        let client_address = socket.local_addr().expect("unable to get the client address");
        println!("connection accepted, client ip is {}", client_address.ip());
        // tokio::spawn(async move {
        //     let (mut rd, mut wr) = socket.split();
        //     if io::copy(&mut rd, &mut wr).await.is_err() {
        //         eprintln!("failed to copy");
        //     }
        // });

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}