<div align="center">
  <img src="https://user-images.githubusercontent.com/71201308/191076633-4efad1ee-c458-4309-886f-c5c3336fecb0.png" width="200">
  
  [![CI](https://github.com/Ubugeeei/Glasper/actions/workflows/rust.yml/badge.svg)](https://github.com/Ubugeeei/Glasper/actions/workflows/rust.yml)
</div>

# Glasper

A toy JavaScript runtime implementation in Rust.

# build and setup

```sh
$ cargo build --release

# set path your shell
$ cp target/release/glasper /usr/local/bin/
$ echo 'export PATH=/usr/local/bin/glasper:$PATH' >> ~/.hogerc
```

# usage

## run source code

```sh
$ glasper example/main.js
```

## run interactive

```sh
$ glasper
Welcome to Glasper v0.1.0
exit using ctrl+c or ctrl+d or exit()
> # input your source
```

## use as library (JavaScript engine)

use engine api (src/engine/api.rs)

```rs
let scope = HandleScope::new();
let context = Context::new(scope);
let mut isolate = Isolate::new(context);
let mut script = Script::compile(String::from("let a = 1;"),  &mut isolate.context.scope);
script.run()
```
