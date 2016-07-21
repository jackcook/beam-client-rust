# beam-client-rust

[![Build Status](https://travis-ci.org/jackcook/beam-client-rust.svg)](https://travis-ci.org/jackcook/beam-client-rust)
[![Coverage Status](https://coveralls.io/repos/github/jackcook/beam-client-rust/badge.svg)](https://coveralls.io/github/jackcook/beam-client-rust)

This is a client library for [Beam](https://dev.beam.pro) written in Rust.

## Quick Start

```rust
extern crate beam;
use beam::Beam;

// The id 252 corresponds to my channel.
static CHANNEL_ID: u32 = 252;

// Print data about my channel, or an error if one occurs.
fn main() {
    let beam = Beam::new();
    let res = beam.channels.get_channel_with_id(CHANNEL_ID);

    match res {
        Ok(channel) => println!("{} has {} viewers.", channel.token, channel.viewersCurrent),
        Err(err) => println!("{}", err),
    }
}
```

## License

beam-client-rust is available under the MIT license. See the LICENSE file for details.
