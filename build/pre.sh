#!/bin/bash
rustup update stable
rustup default stable
rustup target add x86_64-unknown-linux-musl
