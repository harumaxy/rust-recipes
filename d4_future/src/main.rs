extern crate tokio;

use futures::compat::{AsyncRead01CompatExt, AsyncWrite01CompatExt};
use tokio::io::{split, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;

use tokio::prelude::*;
#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let listenr = TcpListener::bind("127.0.0.1:8092").await?;
    'session: loop {
        let (mut socket, _addr) = listenr.accept().await?;
        let (r, w) = socket.split();
        let r = BufReader::new(r);
        let mut lines = r.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            println!("{}", line);
            if line == "quit" {
                break 'session;
            }
        }
    }
    Ok(())
}
