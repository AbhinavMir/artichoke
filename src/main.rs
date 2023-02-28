use arti::tor_client::TorClient;
use futures::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Tor client
    let tor_client = TorClient::new().unwrap();

    // Initiate a connection over Tor to example.com, port 80
    let mut stream = tor_client.connect(("example.com", 80)).await?;

    // Write out an HTTP request
    stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n").await?;

    // IMPORTANT: Make sure the request was written.
    // Arti buffers data, so flushing the buffer is usually required.
    stream.flush().await?;

    // Read and print the result
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
