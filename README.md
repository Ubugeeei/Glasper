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
	console_log("Hello World!");
}

/**
 *
 * primitive types
 *
 */
{
	console_log("true:", true);
	console_log("false:", false);
	console_log("1:", 1);
	console_log("0x1111:", 0x1111);
	console_log("0o1111:", 0o1111);
	console_log("0b1111:", 0b1111);
	console_log("1.1:", 1.1);
	console_log("1.1e3:", 1.1e3);
	console_log("1.1e-3:", 1.1e-3);
	console_log("hello string");
	console_log("undefined:", undefined);
	console_log("null:", null); // (Object)
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
		console_log("variables a:", a);
		a = 5;
		console_log("variables assigned a:", a);
	}

	{
		let b = 2;
		console_log("variables b:", b);
		b = 6;
		console_log("variables assigned b:", b);
		let b = 7;
		console_log("variables re declared b:", b);
	}

	{
		const c = 3;
		console_log("variables c:", c);
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
		console_log("child scope v:", v);
		console_log("global in child:", global);
	}
	console_log("parent scoped v:", v);
}

/**
 *
 * operators
 *
 */
{
	console_log("2 + 2:", 2 + 2);
	console_log("2 - 2:", 2 - 2);
	console_log("2 * 2:", 2 * 2);
	console_log("2 / 2:", 2 / 2);
	console_log("2 % 2:", 2 % 2);
	console_log("2 ** 2:", 2 ** 2);
	console_log("2 << 2:", 2 << 2);
	console_log("2 >> 2:", 2 >> 2);
	console_log("2 & 2:", 2 & 2);
	console_log("2 | 2:", 2 | 2);
	console_log("2 ^ 2:", 2 ^ 2);
	console_log("2 && 2:", 2 && 2);
	console_log("2 || 2:", 2 || 2);
	console_log("2 ?? 2:", 2 ?? 2);
	console_log("2 == 2:", 2 == 2);
	console_log("2 != 2:", 2 != 2);
	console_log("2 > 2:", 2 > 2);
	console_log("2 < 2:", 2 < 2);
	console_log("2 + 2 * 2:", 2 + 2 * 2);
}

/**
 *
 * if statement branch
 *
 */
{
	let num = 2;
	if (num % 2 == 0) {
		console_log("even!");
	} else {
		console_log("odd!");
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

	console_log("add(1, 2):", add(1, 2));
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
	console_log("factorial(5):", factorial(5));
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
			console_log("FizzBuzz");
		} else if (num % 5 == 0) {
			console_log("Buzz");
		} else if (num % 3 == 0) {
			console_log("Fizz");
		} else {
			console_log(num);
		}

		fizzBuzz(num - 1);
	};
	console_log("=== fizzBuzz(20) start ===");
	fizzBuzz(20);
	console_log("=== fizzBuzz(20) end ===");
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
