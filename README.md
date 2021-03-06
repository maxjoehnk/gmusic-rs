# gmusic-rs

[![https://docs.rs/gmusic](https://docs.rs/gmusic/badge.svg)](https://docs.rs/gmusic)
[![Build Status](https://travis-ci.com/maxjoehnk/gmusic-rs.svg?branch=master)](https://travis-ci.com/maxjoehnk/gmusic-rs)

An unofficial client library for Google Music.

Port of [gmusicapi](https://github.com/simon-weber/gmusicapi).

## Usage
Add the following to your Cargo.toml file.
```toml
[dependencies]
gmusic = "0.3"
```

Generate a client id and client secret.

```rust
use gmusic::{GoogleMusicApi, auth::stdio_login};

#[tokio::main]
async fn main() {
    let api = GoogleMusicApi::new(client_id, client_secret, None)?;

    api.login(stdio_login).await?;

    let tracks = api.get_all_tracks().await?;
}
```
