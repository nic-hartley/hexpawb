# HexPawb

HexPawb is a from-scratch implementation of onion routing, using modern technology to .
It natively supports:

- Anonymously connecting to normal servers over the network
- Hosting anonymous services with end-to-end encrypted connections
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

## How's it work?

An explanation of everything from high-level overviews to bit-by-bit dissections of network formats is available in [the wiki](https://github.com/nic-hartley/hexpawb/wiki).
If you know a bit about Tor, it'll look very familiar.
