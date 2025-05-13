# Text2Hex
Converts command-line input (UTF-8) into hexadecimal

> [!WARNING]
> This is solely a pet-project of mine; a short, fun warmup for me to practice
> writing code in Rust. It's not designed for any serious, real-world use. Do
> not expect this project to be maintained properly.
> 
> Use this code (and the associated binaries) at your own discretion.

## Usage
To run the code from the local package:

```
$ cargo run -- 'your input here'
```

To build the binary ONLY:

```
$ cargo build
```

The binary will be located in `target/debug/` under the name `text2hex`. Feel
free to add it to your local binaries as is appropriate for your operating
system and file system configuration.

### Using the binary
The binary has the following usage synopsis:

```
$ text2hex [STRING]
```

As of version `0.1.0`, there are no "option" arguments (e.g. `--help`,
`--version`). This functionality will be added in a later version.
