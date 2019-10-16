# OrbGame

OrbGame is a game extion for [OrbTk](https://gitlab.redox-os.org/redox-os/orbtk.git) to develop 2D games.

[![Build status](https://gitlab.redox-os.org/redox-os/orbgame/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbgame/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Platforms

* Redox OS (native | cargo-node)
* Linux (native | cargo-node)
* macOS (native | cargo-node)
* Windows (native | cargo-node)
* openBSD (not tested, but should work)
* Web (cargo-node)
* Android (native planned after 0.3 | cargo-node)
* iOS (native planned after 0.3 | cargo-node planned after 0.3)
* Ubuntu Touch (native planned  after 0.3 | cargo-node planned for 0.3)

## Usage

To include orbgame in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbgame = { git = "https://gitlab.redox-os.org/redox-os/orbgame.git" }
```
## Minimal Example

```rust
use orbgame::prelude::*;

fn main() {
    Game::new()
        .window(|ctx| {
            Window::create()
                .title("OrbGame - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbGame").build(ctx))
                .build(ctx)
        })
        .run();
}
```

## Run Examples

You can find examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example dungeon --release
```

## Run Examples with cargo-node

To run the examples on as browser, electron or cordova app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start examples

You can start the widgets example by executing the following command:

* Run as browser app:

```text
cargo node run --browser --example dungeon
```

* Run as electron app:

```text
cargo node run --electron --example dungeon
```

* Run as cordova app on android:

```text
cargo node run --android --example dungeon
```

## Build and run documentation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

## Sub Crates

* [api](https://gitlab.redox-os.org/redox-os/orbgame/tree/master/crates/api): additional game elements
* [utils](https://gitlab.redox-os.org/redox-os/orbgame/tree/master/crates/utils): Game helper structs and traits
* [widgets](https://gitlab.redox-os.org/redox-os/orbgame/tree/master/crates/widgets): Game widget library

 ## Credits
 
 * https://pixel-poem.itch.io/dungeon-assetpuck

## License

Licensed under MIT license ([LICENSE](LICENSE)).