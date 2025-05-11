# rnetmap
`rnetmap` is a lightweight, high-performance network port scanner written in Rust. It provides a simple CLI interface to scan individual ports or port ranges to a minimal subset of nmap's functionality.

## Features
* Asynchronous, concurrent scanning of ports, comma-seperated lists, and hyphenated ranges (e.g. `22,80,1000-1010`)

## Installation
Ensure you have Rust and Cargo installed (Rust 1.80+ recommended):
```
rustup --version
cargo --version
```
Clone the repository and build:
```sh
git clone https://github.com/nerbzzz/rnetmap.git
cd rnetmap
cargo build --release
```
The resulting binary will be in `target/release/rnetmap`.

## Basic Usage
```sh
# Single port
rnetmap -p 443 example.com

# Port range
rnetmap -p 1-1024 myserver.local

# Complex port scanning
rnetmap -p 22,80,1000-1024 10.0.0.5
```