# HexPawb

HexPawb is a from-scratch implementation of onion routing, using the concepts of [walking onions](https://www.usenix.org/system/files/sec20-komlo.pdf) to significantly decrease network bandwidth requirements.
It natively supports:

- Anonymously connecting to normal servers over the network
- Hosting anonymous services
- Running as a SOCKS proxy *or* VPN
- Disabling internet access when not connected
- Automatic relay management

If this seems similar to Tor, you're right!
This is not groundbreaking software.
It's a fun project, and tries to do some interesting things.

## What are HexPawb, `hexpawb`, `pawb`, and `furtive`?

HexPawb is a system design and network protocol which provides anonymization through a relay network, like Tor.
It's also the name of the primary HexPawb protocol network.

`hexpawb`, aka `libhexpawb`, is the Rust crate implementing the HexPawb network protocol.
It has bindings in several languages.

`pawb` is the command-line tool using `hexpawb` to create a VPN or SOCKS proxy to access things over the HexPawb network.

`furtive` is the command-line tool using `hexpawb` to host furtive services, its version of [onion services](https://community.torproject.org/onion-services/).

## Why the name?

HexPawb is sort of a knockoff of Tor -- at least in the sense that it serves a similar purpose, but pragmatically it also borrows several features from Tor[.](https://e926.net/posts?tags=paws "I'm a furry, you can guess the real reason.")
So if Tor is Mage Hand, allowing someone to interact at a distance without the affiliation being obvious, then this is Hex Pawb.
