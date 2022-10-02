<div align="center">
  <img src="https://user-images.githubusercontent.com/71201308/191884187-75417bf0-8d23-4d89-8f8a-ba0d1d5e4ab9.png" width="200">

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

### Basic Execution

```rs
use glasper::engine::*;

let handle_scope = HandleScope::new();
let mut context = Context::new(handle_scope);
let mut isolate = Isolate::new(context);
let mut script = Script::compile(String::from("let a = 1;"),  &mut isolate.context);
script.run()
```

### Binding Objects

```rs
use glasper::engine::*;

let handle_scope = HandleScope::new();
let mut context = Context::new(handle_scope);

let global = context.global();
let console_builder = ConsoleBuilder::new();
let console = console_builder.build();
global.set("console", console);

let mut isolate = Isolate::new(context);
let mut script = Script::compile(String::from("log(1, 2, 3);"),  &mut isolate.context);
script.run()
```

<details>
<summary>builtin console sample</summary>

```rs
use glasper::engine::*;

pub struct ConsoleBuilder;
impl Default for ConsoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ConsoleBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> RuntimeObject {
        let mut properties = HashMap::new();
        properties.insert(
            String::from("log"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("debug"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("warn"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );

        RuntimeObject::Object(JSObject { properties })
    }
}

fn log(args: Vec<RuntimeObject>) -> RuntimeObject {
    for arg in args {
        print!("{}", arg);
        print!("\x20");
    }
    println!();

    RuntimeObject::Undefined(JSUndefined)
}
```

</details>
