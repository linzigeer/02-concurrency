use anyhow::Result;
use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

const BUFF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let listener_ip = "0.0.0.0:6379";
    let listener = TcpListener::bind(listener_ip).await?;
    info!("listener working on:{:?}", listener);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        info!("client address:{}", client_addr);
        tokio::task::spawn(process_connect(stream, client_addr));
    }
}

async fn process_connect(mut stream: TcpStream, client_addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUFF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let msg = String::from_utf8_lossy(&buf);
                info!("{}", msg);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Remote Connection {} disconnected!", client_addr);
    Ok(())
}
