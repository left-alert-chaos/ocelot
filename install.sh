#!/bin/sh
echo "This will create an engines directory in your home directory and a directory in ~/.local/share/ocelot-chess"
mkdir -p ~/engines/
cargo build --release
mv ./target/release/ocelot-chess ~/engines/

mkdir -p ~/.local/share/ocelot-chess
touch ~/.local/share/ocelot-chess/uci-log
