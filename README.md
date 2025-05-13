# Text2Hex
![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmattaroni%2Ftext2hex%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&label=version)
![license](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmattaroni%2Ftext2hex%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.license&label=version&color=blueviolet)

Converts command-line input (UTF-8) into hexadecimal

> [!WARNING]
> This is solely a pet-project of mine; a short, fun warmup for me to practice
> writing code in Rust. It's not designed for any serious, real-world use. Do
> not expect this project to be maintained properly.
> 
> Use this code (and the associated binaries) at your own discretion.

## Download
You can download the Text2Hex binary from the [releases page] on GitHub.

## Build from source
Text2Hex requires the following tools to build:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/downloads)

Clone the repository with `git`, then compile the binary with `cargo`.

```
git clone --depth 1 https://github.com/mattaroni/text2hex.git
cd text2hex/
cargo build
```

The binary will be located in `target/debug/` under the name `text2hex`.

## Usage
The binary has the following usage synopsis:

```
text2hex [STRING]
```

As of version `0.1.0`, there are no "option" arguments (e.g. `--help`,
`--version`). This functionality will be added in a later version.

[releases page]: https://github.com/mattaroni/text2hex/releases
