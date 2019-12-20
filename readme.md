# `tbot`

Make cool Telegram bots with Rust easily. For example, here's a simple echo bot:

```rust
use tbot::prelude::*;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {
            let echo = &context.text.value;
            let call_result = context.send_message(echo).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
```

If you're a newcomer, we recommend you go through the [tutorial] first. We also
have several [How-to guides][how-to] to help you use `tbot`.

If you have a question, ask it in [our group] on Telegram. If you find a bug,
fill an issue on either our [GitLab] or [GitHub] repository.

> **Important note:** `tbot` v0.3's minimum supported Rust version is 1.40
> due to use of `#[non_exhaustive]`.

## Features

- Full Bots API support, including media upload/download and recent API updates;
- Support for both polling and [webhooks];
- Type-safe API;
- Based on futures and `tokio`;
- Easy to use, while scalable and configurable.

## Installing

Add `tbot` to your Cargo.toml:

```toml
[dependencies]
tbot = "0.3"
```

## Documentation

There are many examples in the [`examples`] directory to see `tbot` in action.
If you want to get started with `tbot`, go through the [tutorial]. When you
start making your bot, our [How-to guides][how-to] may help you. And you can
always refer to our API docs on [_docs.rs_][api-docs] (docs for `master` are
also available [here][master-docs]).

## Contribution

Glad you want to contribute to `tbot`! We develop the crate on [GitLab], so
create your pull/merge request there if you can. We accept pull requests on
[GitHub] as well, but we prefer [GitLab].

[our group]: https://t.me/tbot_group
[webhooks]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
[tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
[how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
[gitlab]: https://gitlab.com/SnejUgal/tbot
[github]: https://github.com/SnejUgal/tbot
[`examples`]: ./examples/
[api-docs]: https://docs.rs/tbot
[master-docs]: https://snejugal.gitlab.io/tbot/tbot/index.html
