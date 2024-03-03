# Bevy Discord Presence Plugin

<!-- [![crates.io](https://img.shields.io/crates/v/bevy-discord-presence)](https://crates.io/crates/bevy-discord-presence)
[![crates.io](https://img.shields.io/crates/d/bevy-discord-presence)](https://crates.io/crates/bevy-discord-presence)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/book/plugin-development/#main-branch-tracking)
[![docs.rs](https://img.shields.io/docsrs/bevy-discord-presence/latest)](https://docs.rs/bevy-discord-presence) -->

> This is a fork of the original [bevy-discord-presence](https://github.com/jewlexx/bevy-discord-rpc) crate, which is no longer maintained.

A simplistic bevy plugin for discord presence integration within the bevy game engine

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy-discord-presence = { git = "https://github.com/EmmettJayhart/bevy-discord-rpc" }
```

## Example

```rust
use bevy::prelude::*;
use bevy_discord_presence::{ActivityState, RPCPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Pass Discord App ID, and set show_time to true.
    app.add_plugins(RPCPlugin::new(965125975941709834, true))
        .add_systems(Update, update_presence);

    app.run();
}

fn update_presence(mut state: ResMut<ActivityState>) {
    state.instance = Some(true);
    state.details = Some("Hello World".to_string());
    state.state = Some("This is state".to_string());
}
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

## Contributions

Contributions to this project are welcome, just follow these steps.

1. Fork this repository and create a feature branch named after the feature you want to implement
2. Make your changes on your branch
3. Add some test if possibe
4. Make sure all tests pass
5. Submit a PR/MR on GitHub

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as described below, without any additional terms or conditions.

## License

Portions of this project, were originally released into the public domain as described by [UNLICENSE](UNLICENSE) by the original developer of the [bevy-discord-presence](https://github.com/jewlexx/bevy-discord-rpc) crate. These portions remain in the public domain, and their use is unrestricted.

Except where noted, all contributions to this repository made after forking from the original repository is dual-licensed under either:

-   MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
However, the project as a whole can also be used under the terms of the MIT License or the Apache License 2.0, at the user's discretion.
