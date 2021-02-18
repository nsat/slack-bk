# slack-bk [![Build Status]][actions] [![Docs]][docs.rs] [![Latest Version]][crates.io]


[Build Status]: https://img.shields.io/github/workflow/status/nsat/slack-bk/Rust/master
[actions]: https://github.com/nsat/slack-bk/actions?query=branch%3Amaster
[Docs]: https://docs.rs/slack-bk/badge.svg
[docs.rs]: https://docs.rs/slack-bk/
[Latest Version]: https://img.shields.io/crates/v/slack-bk.svg
[crates.io]: https://crates.io/crates/slack-bk

Rust crate for Slack's BlockKit API

You'll probably want to reference [Slack's documentation](https://api.slack.com/block-kit) while using this crate.

## Using `slack-bk` with an HTTP client

`slack-bk` does not come with a built in mechanism to talk to slack's API. There are many popular
HTTP libraries in the rust ecosystem and the user is free to choose their own.

```rust
use reqwest::{Client, Error};
use slack_bk::surfaces::Message;

async fn send_to_webhook(webhook: &str, client: &Client, msg: Message) -> Result<(), Error> {
    client
        .post(webhook)
        .json(&msg)
        .send()
        .await?
        .error_for_status()?
        .map(|_| ())

}
```
