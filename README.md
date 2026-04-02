# wyzie-subs

[![crates.io](https://img.shields.io/crates/v/wyzie-subs.svg)](https://crates.io/crates/wyzie-subs)
[![Documentation](https://docs.rs/wyzie-subs/badge.svg)](https://docs.rs/wyzie_subs)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/wyzie-subs.svg)](./LICENSE-APACHE)

A client for downloading subtitles from [sub.wyzie.ru](https://sub.wyzie.ru).

## Features

- `utoipa-impl` - Implements [ToSchema](https://docs.rs/utoipa/latest/utoipa/trait.ToSchema.html) on data structures
- `schemars-impl` - Implements [JsonSchema](https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html) on data structures

## Usage

```rust
use reqwest::Client;
use url::Url;
use wyzie_subs::{WyzieClient, models::SearchParams};

#[tokio::main]
async fn main() {
    let wyzie = WyzieClient::builder()
        .base_url(Url::parse("https://sub.wyzie.ru").unwrap())
        .reqwest_client(Client::new())
        .build();

    let params = SearchParams::builder()
        .id("93740".to_string())
        .season(1)
        .episode(1)
        .key(std::env::var("API_KEY").unwrap())
        .build();

    let subtitles = wyzie.search(&params).await.unwrap();
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
