# Cellular automata
This is a simple Rust library with associated command-line executable to play with 1-dimension cellular automata.

## Examples
### text output in the terminal
```sh
automata-cli --rule 30 --steps=10 --width=20
```
```
|          *         |
|         ***        |
|        **  *       |
|       ** ****      |
|      **  *   *     |
|     ** **** ***    |
|    **  *    *  *   |
|   ** ****  ******  |
|  **  *   ***     * |
| ** **** **  *   ***|
```

### image output
```sh
automata-cli --rule 82 --steps 320  --width 640 --output rule82.320.png
```
![alt text](https://github.com/Bobox214/rs-cellular-automata/blob/master/outputs/rule82.320.png "Rule 82 320 steps")

## Compiling & Running From Source
### Prerequisites

* [rust](https://www.rust-lang.org)

### Compiling

```sh
cargo build --release
```

### Running CLI

```sh
cargo run --release --bin automata-cli -- --help
```

```sh
./target/release/automata-cli --help


