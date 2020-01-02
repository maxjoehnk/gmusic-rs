# gmusic-rs
An unofficial client library for Google Music.

Port of [gmusicapi](https://github.com/simon-weber/gmusicapi).

## Usage
Add the following to your Cargo.toml file.
```toml
[dependencies]
gmusic = "0.1"
```

Generate a client id and client secret.

```rust
use gmusic::{GoogleMusicApi, auth::stdio_login};

fn main() {
    let api = GoogleMusicApi::new(client_id, client_secret)?;

    api.login(stdio_login)?;

    let tracks = api.get_all_tracks()?;
}
```