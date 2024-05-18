# rust-game-tetris

Tetris game (Rust)

![Tetris!](./readme-contents/imgs/ss-1.png)

### Keys:

A: Move left

D: Move right

S: Faster move down

W: Change direction

## Installation

Use [Rust-installation](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install Rust.
Or [Rust-other-installation-methods](https://forge.rust-lang.org/infra/other-installation-methods.html) if you are using windows.

Go to the game-tetris folder

```bash
cd game-tetris
```

Install deps

```bash
cargo install --path .
```

## Usage

Run inside game-tetris folder

```bash
cargo run
```

## Wanna faster?

Open `./game-tetris/src/config.rs` and change `GAME_SPEED` value:

example

```rust
pub const GAME_SPEED: u16 = 60;
```

To

```rust
pub const GAME_SPEED: u16 = 10;
```

or anything

### License

MIT. >>> [LICENSE](LICENSE)
