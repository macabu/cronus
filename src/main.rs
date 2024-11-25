use either::Either::{Left, Right};
use packet::PacketHeader;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod packet;
mod service;

#[tokio::main]
async fn main() {
    if let Err(e) = tokio::spawn(run()).await {
        eprintln!("main error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            // TODO: do not create this for every connection?
            let login_service = service::LoginService {};

            // TODO: too much happening inside this accept loop.
            loop {
                // Peek header.
                let mut header_buf = [0; 2];
                match socket.peek(&mut header_buf).await {
                    Ok(n) => debug_assert!(n == 2),
                    Err(e) => {
                        eprintln!("Failed to read header: {:?}", e);
                        return;
                    }
                }

                // Match packet type.
                let header = i16::from_le_bytes(header_buf);
                match header {
                    packet::PACKET_CA_LOGIN::HEADER => {
                        // Read exactly the expected packet size.
                        let mut buf = [0; 55]; // TODO: move this somewhere. what about varying length packets?
                        socket.read_exact(&mut buf).await.expect("failed to parse");

                        // Convert to structured data (domain).
                        let packet = packet::PACKET_CA_LOGIN::try_from(&buf[..])
                            .expect("failed to parse packet");

                        println!("[{}] Received packet {:?}", addr.ip(), packet);

                        // Invoke command handler that may return a response.
                        let response = login_service
                            .login(&packet)
                            .expect("failed to handle login");

                        // Write response into the socket in case it exists.
                        let src: Vec<u8> = match response {
                            Left(success) => success.into(),
                            Right(failure) => failure.into(),
                        };

                        socket
                            .write_all(&src)
                            .await
                            .expect("failed to write response");
                    }
                    _ => {
                        eprintln!("[{}] Packet {:?} not implemented!", addr.ip(), header_buf);
                    }
                }
            }
        });
    }
}
