#!/bin/sh
echo "This will create an engines directory in your home directory."
mkdir -p ~/engines/
cargo build --release
mv ./target/release/ocelot-chess ~/engines/
