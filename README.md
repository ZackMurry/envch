# envch

![Screenshot of envch in the terminal](assets/readme-simple-example.png)

An intuitive program that allows users to create, modify, list, and remove environment variables

## Installation

### Cargo
If you don't have Cargo already, install it using this command:

#### macOS and Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows
If you're on Windows, you can install Cargo using [this guide](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).


#### Install envch

Then, install envch using:
```bash
cargo install envch
```

### Manually
To manually install envch, run the following:
```bash
git clone https://github.com/ZackMurry/envch.git
cd envch

cargo build --release

cd target/release
./envch
```
