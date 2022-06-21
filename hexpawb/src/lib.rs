/*!
This crate presents a clean and simple interface to the HexPawb network.
If you wanted to, say, load `https://www.torproject.org/` through HexPawb,
you'd just need to:

```no_run
use hexpawb::Network;
// First you need to connect to the HexPawb network
let mut network = Network::connect().await
    .expect("Failed to join network");
// Then you can create a circuit
let mut circuit = network.circuit().await
    .expect("Failed to build circuit");
// Then you can connect to something over that circuit (this does DNS
// through the circuit, but to your system's configured DNS servers)
let mut connection = circuit.tcp("www.torproject.org:80").await
    .expect("Failed to connect circuit");
// TCP connections work basically just like the stdlib's TcpStream,
// but async.
connection.send("GET / HTTP/1.1\r\nConnection: close\r\n\r\n").await
    .expect("Failed to send request");
std::io::copy(circuit, std::io::stdout().lock()).await
    .expect("Failed to receive body");
```

# Breaking privacy

***If and only if*** you know exactly what you're doing and you can state
in clear, uncertain terms precisely why you need to do it, you can enable
the lower-level API with feature `dangerous-low-level-bits`. If you don't
use it exactly right, you'll break your own anonymity irrecoverably. This
enables functionality like:

```ignore
use hexpawb::Network;
let mut network = Network::builder()
    .authority(custom_authority)
    .connect().await
    .expect("Failed to join network");
let mut circuit = network.tcp("www.torproject.org:90")
    .length(10)
    .relay(specific_relay)
    .connect().await
    .expect("Failed to connect circuit");
circuit.send("GET / HTTP/1.1\r\nConnection: close\r\n\r\n").await
    .expect("Failed to send request");
std::io::copy(circuit, std::io::stdout().lock()).await
    .expect("Failed to receive body");
```
 */

use std::net::{IpAddr, SocketAddr};

#[derive(Debug)]
pub enum PawbError {}

pub type PawbResult<T> = Result<T, PawbError>;

/*
#[cfg(feature = "rt-sync")]
pub fn
#[cfg(not(feature = "rt-sync"))]
pub async fn
*/

/**
Represents the current known state of the HexPawb network. Allows you to
build new circuits to arbitrary IPs, look up and connect to furtives, etc.

Unless you're in the dangerous mode, the only way to connect is with the
[`connect`](Network::connect) method.

Once you're connected to the network, you can establish a circuit with
[`circuit`](Network::circuit), which handles everything for you.
*/
pub struct Network {}
impl Network {
    /**
    Connect into the HexPawb network. This will download the necessary
    connection information and a starter set of relays to connect into.
    */
    #[cfg(feature = "rt-sync")]
    pub fn connect() -> PawbResult<Network> {
        NetworkBuilder::standard().connect()
    }

    #[cfg(not(feature = "rt-sync"))]
    pub async fn connect() -> PawbResult<Network> {
        NetworkBuilder::standard().connect().await
    }

    /**
    Build your own custom network with your own custom configuration. This
    carries a high risk of breaking your own anonymity -- HexPawb assumes you
    have a crowd to blend in with, and on your own network you just don't.
    */
    #[cfg(feature = "dangerous-low-level-bits")]
    pub fn builder() -> NetworkBuilder {
        todo!()
    }

    /**
    The authorities that this network is trusting to define the state.
    */
    pub fn authorities(&self) -> &[Authority] {
        todo!()
    }

    /**
    Create a circuit through the network.

    Be thoughtful about the circuits you make and what you use them for. One
    lone circuit is almost never enough, but one per connection is usually far
    too many. See the [`Circuit`] documentation for more details.
    */
    #[cfg(feature = "rt-sync")]
    pub fn circuit(&self) -> PawbResult<Circuit> {
        todo!()
    }
    #[cfg(not(feature = "rt-sync"))]
    pub async fn circuit(&self) -> PawbResult<Circuit> {
        todo!()
    }
}

/**
Builder-pattern struct for constructing custom networks. Construct one with
[`Network::builder`].
*/
pub struct NetworkBuilder {}
impl NetworkBuilder {
    fn new() -> Self {
        todo!()
    }

    /**
    Add an authority to this network you're building.
    */
    pub fn authority(self, _authority: Authority) -> Self {
        todo!()
    }

    /**
    Actually reach out and start connecting to this network.
    */
    #[cfg(feature = "rt-sync")]
    pub fn connect(self) -> PawbResult<Network> {
        todo!()
    }
    #[cfg(not(feature = "rt-sync"))]
    pub async fn connect(self) -> PawbResult<Network> {
        todo!()
    }

    /**
    The configuration to connect to the HexPawb network.
    */
    fn standard() -> Self {
        Self::new()
        // .authority(...)
        // .authority(...)
    }
}

/**
A directory authority on the HexPawb network. Used primarily to validate that
various items are properly authenticated.
*/
pub struct Authority {}

/**
A single path through the HexPawb network, which has been set up and is ready
to have traffic flow over it.

# Choosing Circuits

***This is important***. Please read it fully. I know it's a lot.

Deciding when to create a new circuit is difficult. You could make one, total,
and use that for everything; that leaves you bottlenecked at the bandwidth of
the circuit and vulnerable to traffic correlation. You could make one for each
new connection, but that means every single connection takes ages to start,
which just won't work if you're for example rendering a modern webpage.

As a rule of thumb, treat circuits like their own independent connections to
the internet, and use them to isolate things that should be isolated. Users
should be separated, individual actions should be separated, etc.

However, this decision is ultimately protocol-dependent and fundamentally not
easy. To make the best decision, you'll want to keep in mind:

- What data can an attacker in the middle discover?
  - TLS can expose hostnames
  - The computer's DNS server may be identifiable to varying degrees
  - The protocol you're sending might not even be encrypted
- If an attacker sees all that coming from one IP, what do they learn?
  - If the same accounts are always accesssed from the same IP, even if that
    address is a HexPawb exit they can still be correlated
- What's the best way to spread out the traffic source to avoid that?

It may help to get out a notebook -- digital or physical -- and spend a day or
two hunting all this information down.
*/
pub struct Circuit {}
impl Circuit {
    #[cfg(feature = "rt-sync")]
    pub fn dns(&mut self, _name: &str) -> PawbResult<Vec<IpAddr>> {
        todo!()
    }
    #[cfg(not(feature = "rt-sync"))]
    pub async fn dns(&mut self, _name: &str) -> PawbResult<Vec<IpAddr>> {
        todo!()
    }

    #[cfg(feature = "rt-sync")]
    pub fn tcp(&mut self, _dest: SocketAddr) -> PawbResult<PawbTcpStream> {
        todo!()
    }
    #[cfg(not(feature = "rt-sync"))]
    pub async fn tcp(&mut self, _dest: SocketAddr) -> PawbResult<PawbTcpStream> {
        todo!()
    }
}

#[cfg(feature = "rt-tokio")]
mod tcpstream {
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::UdpSocket,
    };

    pub struct Stream {
        socket: UdpSocket,
    }
    impl AsyncRead for Stream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            todo!()
        }
    }
    impl AsyncWrite for Stream {
        fn poll_write(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<Result<usize, std::io::Error>> {
            todo!()
        }
        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            todo!()
        }
        fn poll_shutdown(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            todo!()
        }
    }
}
#[cfg(feature = "rt-async-std")]
mod tcpstream {
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::UdpSocket,
    };

    pub struct Stream {
        socket: UdpSocket,
    }
    impl AsyncRead for Stream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            todo!()
        }
    }
    impl AsyncWrite for Stream {
        fn poll_write(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<Result<usize, std::io::Error>> {
            todo!()
        }
        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            todo!()
        }
        fn poll_shutdown(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            todo!()
        }
    }
}

// if they dont' enable an async runtime, default to sync
#[cfg(feature = "rt-sync")]
mod tcpstream {
    use std::{
        io::{self, Read, Write},
        net::UdpSocket,
    };

    pub struct Stream {
        #[allow(unused)]
        socket: UdpSocket,
    }
    impl Stream {
        pub fn nonblocking(&self) -> bool {
            todo!()
        }
        pub fn set_nonblocking(&mut self, _val: bool) {
            todo!()
        }
    }
    impl Read for Stream {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            todo!()
        }
    }
    impl Write for Stream {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            todo!()
        }
        fn flush(&mut self) -> io::Result<()> {
            todo!()
        }
    }
}

/**
A TCP stream that's going over HexPawb.

This is meant to be *mostly* source-compatible with `tokio::net::TcpStream` or
`async_std::net::TcpStream`, though some options (notably, `TCP_NODELAY`) are
unavailable for technical reasons.

Depending on your features this implements different traits:
- `futures`: `futures::io::AsyncRead`
- `async-std`: `async_std::io::Read`
- `tokio`: `tokio::io::AsyncRead`
*/
pub use tcpstream::Stream as PawbTcpStream;
