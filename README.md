# Cellular automata
This is playground project to tackle the Rust language.
The goal is to play around with 1-dimension cellular automata with different kind of user interaction, CLI, GUI or WEB.

The workspace consist in 4 crates:
- automata-lib : The main library that contains the automata computation and its rules.
- automata-cli : a CLI interface to automata-lib with output as text or direct image file.
- automata-gtk : a GUI interface to automata-lib in GTK.
- automata-wasm : A web page using WASM for interfacing with automata-lib and javascript for gui part

# Compiling

## Prerequisites

* [rust](https://www.rust-lang.org)
* [wasm-pck](https://rustwasm.github.io/wasm-pack/installer/)
* [npm](https://www.npmjs.com/get-npm)

## Compiling
The compilation for automata-lib, automata-cli and automata-gtk is simply using cargo build command:
```sh
cargo build --release
```
The compilation for automata-wasm requires using wasm-pack:
```sh
cd automata-wasm
wasm-pack build
```

# CLI
Help for all available option 
```sh
automata-cli --help
```
## text output in the terminal
```sh
automata-cli --colors 2 --rule 30 --steps=10 --width=20
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

## image output
```sh
automata-cli --colors 4 --rule 16673 --steps 320  --width 640 --output 4C_16673.320.png
```
![alt text](https://github.com/Bobox214/rs-cellular-automata/blob/master/outputs/4C_16673.320.png "Colors 4 Rule 16673 320 steps")

# GUI
GUI with all controls is simply launched with :
```sh
automata-gtk
```

![alt text](https://github.com/Bobox214/rs-cellular-automata/blob/master/outputs/Screenshot-gtk.png "GTK GUI")

# WASM

To start the web-page using npm , after building with wasm-pack :
```sh
cd automata-wasm/www
npm install
npm run star
```
You can then open your browser on localhost:8080/ and play with cellular automata !

![alt text](https://github.com/Bobox214/rs-cellular-automata/blob/master/outputs/Screenshot-wasm.png "Web View")





