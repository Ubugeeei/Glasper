<div align="center">
  <img src="https://user-images.githubusercontent.com/71201308/191806164-dd912fe2-266b-447b-813b-6b8a93ad1391.png" width="200">

  [![CI](https://github.com/Ubugeeei/Glasper/actions/workflows/rust.yml/badge.svg)](https://github.com/Ubugeeei/Glasper/actions/workflows/rust.yml)

A toy JavaScript engine and runtime implementation in Rust.
</div>

# Installation

```sh
$ make install
# set path your shell
$ echo 'export PATH=/usr/local/bin/gls:$PATH' >> ~/.hogerc
```

# Usage

## Run source code
```sh
$ gls example/main.js
```

## Run interactive

```sh
$ gls
Welcome to Glasper v0.1.0
exit using ctrl+c or ctrl+d or exit()
> # input your source
```

you can get more info by help command.
```sh
$ gls --help
```
## Use as library (JavaScript engine)

use engine api (src/engine/api.rs)

```rs
let handle_scope = HandleScope::new();
let mut context = Context::new(handle_scope);
let mut isolate = Isolate::new(context);
let mut script = Script::compile(String::from("let a = 1;"),  &mut isolate.context);
script.run()
```

binding builtin objects
```rs
let handle_scope = HandleScope::new();
let mut context = Context::new(handle_scope);

// bind to global scope
let global = context.global();
// make function object (my_logger)
let log = Object::BuiltinFunction(GBuiltinFunction::new("log", my_logger));
// set
global.set("log", log);

let mut isolate = Isolate::new(context);
let mut script = Script::compile(String::from("log(1, 2, 3);"),  &mut isolate.context);
script.run()
```
