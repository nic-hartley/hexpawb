use std::{
    env,
    io::{self, Read, Write},
    net::SocketAddr,
    sync::mpsc::{sync_channel, TryRecvError},
    thread,
    time::Duration,
};

use hexpawb::Network;

fn main() {
    // For now, basically netcat but through pawb
    let mut args = env::args().skip(1);
    let dest_name = args.next().expect("provide a destination");
    let dest_port = args
        .next()
        .map(|s| s.parse::<u16>().ok())
        .flatten()
        .unwrap_or(80);
    let network = Network::connect().expect("Failed to connect to HexPawb");
    let mut circuit = network.circuit().expect("Failed to create circuit");
    let dest_addr = circuit
        .dns(&dest_name)
        .expect("Failed to DNS destination")
        .into_iter()
        .next()
        .expect("Did not get any IP addresses");

    let (stdin_s, stdin_r) = sync_channel(8);
    let inp_thread = std::thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let mut buf = vec![0; 16384];
        loop {
            let amt = stdin.read(&mut buf).expect("Failed to read input data");
            stdin_s
                .send(buf[..amt].to_vec())
                .expect("Failed to handle input data");
        }
    });

    let mut cxn = circuit
        .tcp(SocketAddr::new(dest_addr, dest_port))
        .expect("Failed to connect over circuit");
    cxn.set_nonblocking(true);
    let mut cxn_buf = vec![0; 16384];
    let mut stdout = io::stdout().lock();

    loop {
        match cxn.read(&mut cxn_buf) {
            Ok(amt) => {
                stdout
                    .write_all(&cxn_buf[..amt])
                    .expect("Failed to write downloaded data");
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => match stdin_r.try_recv() {
                Ok(data) => {
                    cxn.write_all(&data).expect("Failed to upload input data");
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            },
            other => {
                other.expect("Failed to read downloaded data");
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    inp_thread.join().expect("Failed to join thread");
}
