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

<details>
<summary>able syntaxes</summary>

```js
/**
 *
 * std out
 *
 */
{
	console.log("Hello World!");
}

/**
 *
 * primitive types
 *
 */
{
	console.log("true:", true);
	console.log("false:", false);
	console.log("1:", 1);
	console.log("0x1111:", 0x1111);
	console.log("0o1111:", 0o1111);
	console.log("0b1111:", 0b1111);
	console.log("1.1:", 1.1);
	console.log("1.1e3:", 1.1e3);
	console.log("1.1e-3:", 1.1e-3);
	console.log("hello string");
	console.log("undefined:", undefined);
	console.log("null:", null); // (RuntimeObject)
}

/**
 *
 * objects
 *
 */
{
	let o = {
		message: "hello object",
	};
	console.log("o.message:", o.message);
}

/**
 *
 * variables
 *
 */
{
	{
		// var
		a = 1;
		console.log("variables a:", a);
		a = 5;
		console.log("variables assigned a:", a);
	}

	{
		let b = 2;
		console.log("variables b:", b);
		b = 6;
		console.log("variables assigned b:", b);
		let b = 7;
		console.log("variables re declared b:", b);
	}

	{
		const c = 3;
		console.log("variables c:", c);
		// c = 7; // error
		// const c = 7; // error
	}
}

/**
 *
 * scope
 *
 */
{
	let v = 1;
	let global = 100;
	{
		let v = 2;
		console.log("child scope v:", v);
		console.log("global in child:", global);
	}
	console.log("parent scoped v:", v);
}

/**
 *
 * operators
 *
 */
{
	console.log("2 + 2:", 2 + 2);
	console.log("2 - 2:", 2 - 2);
	console.log("2 * 2:", 2 * 2);
	console.log("2 / 2:", 2 / 2);
	console.log("2 % 2:", 2 % 2);
	console.log("2 ** 2:", 2 ** 2);
	console.log("2 << 2:", 2 << 2);
	console.log("2 >> 2:", 2 >> 2);
	console.log("2 & 2:", 2 & 2);
	console.log("2 | 2:", 2 | 2);
	console.log("2 ^ 2:", 2 ^ 2);
	console.log("~-1:", ~-1);
	console.log("2 && 2:", 2 && 2);
	console.log("2 || 2:", 2 || 2);
	console.log("2 ?? 2:", 2 ?? 2);
	console.log("2 == 2:", 2 == 2);
	console.log("2 != 2:", 2 != 2);
	console.log("2 === 2:", 2 === 2);
	console.log("2 !== 2:", 2 !== 2);
	console.log("2 > 2:", 2 > 2);
	console.log("2 < 2:", 2 < 2);
	console.log("2 <= 2:", 2 <= 2);
	console.log("2 >= 2:", 2 >= 2);
	console.log("2 + 2 * 2:", 2 + 2 * 2);
	console.log("typeof 1:", typeof 1);
}

/**
 *
 * if statement branch
 *
 */
{
	let num = 2;
	if (num % 2 == 0) {
		console.log("even!");
	} else {
		console.log("odd!");
	}
}

/**
 *
 * function
 *
 */
{
	const add = function (a, b) {
		return a + b;
	};

	console.log("add(1, 2):", add(1, 2));
}

/**
 *
 * recursive function
 *
 */
{
	const factorial = function (num) {
		if (num == 0) {
			return 1;
		} else {
			return num * factorial(num - 1);
		}
	};
	console.log("factorial(5):", factorial(5));
}

/**
 *
 * fizzBuzz example
 *
 */
{
	const fizzBuzz = function (num) {
		// comment out
		if (!num) return 0;

		if (num % 15 == 0) {
			console.log("FizzBuzz");
		} else if (num % 5 == 0) {
			console.log("Buzz");
		} else if (num % 3 == 0) {
			console.log("Fizz");
		} else {
			console.log(num);
		}

		fizzBuzz(num - 1);
	};
	console.log("=== fizzBuzz(20) start ===");
	fizzBuzz(20);
	console.log("=== fizzBuzz(20) end ===");
}
```

</details>

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
