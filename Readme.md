# Hello World Rust

## Intro

Hello World to Rust

## Contents

- [Install](#install)
- [Create New Project](#create-new-project)

## Install

`asdf` rocks:

```bash
asdf plugin-add rust https://github.com/code-lever/asdf-rust.git
asdf install rust 1.61.0
asdf global rust 1.61.0
```

## Create New Project

```bash
# create new project
cargo new http-client

# write code

# compile debug version and execute
cargo build
./target/debug/http-client

# or composition of those commands
cargo run

# when ready for production
cargo build --release
```
