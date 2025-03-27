#!/usr/bin/env bash
#Github actions doesnt like me so i guess i have to use thiss
#Using https://github.com/cross-rs/cross
cross build --target x86_64-pc-windows-gnu --release
cross build --target x86_64-unknown-linux-musl --release
#Todo: Add macos builds