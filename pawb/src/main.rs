use std::{env, net::SocketAddr};

use futures::{AsyncReadExt as _, AsyncWriteExt as _};
use hexpawb::Network;
use tokio::io::{self, AsyncWriteExt as _, AsyncReadExt as _};

#[tokio::main]
async fn main() {
    // For now, basically netcat but through pawb
    let mut args = env::args().skip(1);
    let dest_name = args.next().expect("provide a destination");
    let dest_port = args.next().map(|s| s.parse::<u16>().ok()).flatten().unwrap_or(80);
    let network = Network::connect()
        .await
        .expect("Failed to connect to HexPawb");
    let mut circuit = network.circuit().await.expect("Failed to create circuit");
    let dest_addr = circuit.dns(&dest_name).await
        .expect("Failed to DNS destination")
        .into_iter().next()
        .expect("Did not get any IP addresses");
    let mut cxn = circuit.tcp(SocketAddr::new(dest_addr, dest_port)).await
        .expect("Failed to connect over circuit");
    let mut cxn_buf = vec![0; 16384];

    let mut stdin = io::stdin();
    let mut stdin_buf = vec![0; 16384];

    let mut stdout = io::stdout();
    loop {
        tokio::select! {
            amount = cxn.read(&mut cxn_buf) => {
                let amount = amount
                    .expect("Failed to download data");
                stdout.write_all(&cxn_buf[..amount]).await
                    .expect("Failed to write downloaded data");
            }
            amount = stdin.read(&mut stdin_buf) => {
                let amount = amount
                    .expect("Failed to read uploadable data");
                cxn.write_all(&stdin_buf[..amount]).await
                    .expect("Failed to upload data");
            }
        }
    }
}
