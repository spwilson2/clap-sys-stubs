# clap-sys-stubs
Rust stub implementation of [clap-sys](https://github.com/glowcoil/clap-sys) callbacks for the [CLAP](https://cleveraudio.org/) audio plugin format.

## Usage

Just fork this repo and get started working on your CLAP project in rust :clap:

## Generating Stubs

This project uses the [quick_gen](quick_gen.py) python3 script to generate
almost the entirety of the bindings. (Only manual editing of the
[surround.rs](src/ext/draft/surround.rs) is required after automatic
generation.) This script is known to work with `python 3.8.9` on MacOS.