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
let mut script = Script::compile(String::from("console.log(1, 2, 3);"),  &mut isolate.context);
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

# Syntax

## Console log

```js
console.log("Hello World!");
```

## Expression

### Primitive literals

```js
// bool
console.log(true);
console.log(false);

// number
console.log(1);
console.log(0x1111);
console.log(0o1111);
console.log(0b1111);
console.log(1.1);
console.log(1.1e3);
console.log(1.1e-3);

// string
console.log("hello string");

// undefined, null
console.log(undefined);
console.log(null);
```

### Operators

```js
console.log(2 + 2);
console.log(2 - 2);
console.log(2 * 2);
console.log(2 / 2);
console.log(2 % 2);
console.log(2 ** 2);
console.log(2 + 2 * 2);

// comp
console.log(2 == 2);
console.log(2 != 2);
console.log(2 === 2);
console.log(2 !== 2);
console.log(2 > 2);
console.log(2 < 2);
console.log(2 <= 2);
console.log(2 >= 2);

// bit
console.log(2 << 2);
console.log(2 >> 2);
console.log(2 & 2);
console.log(2 | 2);
console.log(2 ^ 2);

// bool
console.log(2 && 2);
console.log(2 || 2);

// nullish
console.log(1 ?? 2); // 1
console.log(null ?? 2); // 2

// typeof
console.log(typeof 1); // "number"
```

## Array and prototype functions

```js
const arr = [1, 2, 3, 4, 5];

console.log(arr[0]); // 1
console.log(arr[100]); // undefined
console.log(arr.length); // 5

const last = arr.at(-1);
console.log(last); // 5

const concat = arr.concat([6, 7, 8]);
console.log(concat[6]); // 6

const every = arr.every(function (v) {
  return v < 10;
});
console.log(every); // true

const joined = arr.join("/");
console.log(joined); // "1/2/3/4/5/6/7/8"

const mapped = arr.map(function (it) {
  return it * 2;
});
console.log(mapped[0]); // 2

const found = arr.find(function (it) {
  return it % 2 === 0;
});
console.log(found); // 2
```

## Object

```js
let o = {
  message: "hello object",
};
console.log(o.message);

o.message = "hello object again";
console.log(o.message);

let o_cp = o;
o_cp.message = "hello object again by copy";
console.log(o.message);
```

## Variable snd scope

### declare "var"

```js
a = 1;
console.log("variables a:", a); // 1
a = 5;
console.log("variables assigned a:", a); // 5
```

### declare "let"

```js
let a = 2;
console.log("variables a:", a); // 2
a = 6;
console.log("variables assigned a:", a); // 6
```

### declare "const"

```js
const c = 3;
console.log("variables c:", c); // 3
c = 7; // error
const c = 7; // error
```

### scope

```js
let v = 1;
let global = 100;
{
  let v = 2;
  console.log("child scope v:", v); // 2
  console.log("global in child:", global); // 100
}
console.log("parent scoped v:", v); // 1
```

## Function

```js
const add = function (a, b) {
  return a + b;
};

console.log("add(1, 2):", add(1, 2));
```

```js
// recursive
const factorial = function (num) {
  if (num == 0) return 1;
  return num * factorial(num - 1);
};
console.log("factorial(5):", factorial(5)); // 120
```

## "this" in Function

```js
const inner_f = function () {
  return this.value;
};
console.log(inner_f()); // undefined

const o = {
  value: 1,
  f: inner_f,
};
console.log(o.f()); // { value: 1, f: [Function] }

const oo = {
  value: 2,
  f: inner_f,
};
console.log(oo.f()); // { value: 2, f: [Function] }
```

## Prototype

```js
Array.prototype = {
  nth: function (n) {
    return this[n];
  },
};

const arr = [1, 2, 3, 4, 5];
console.log(arr.nth(2)); // 3
```

## Statements

### If

```js
let num = 2;
if (num % 2 == 0) {
  console.log("even!");
} else {
  console.log("odd!");
}
```

### switch

```js
const match = function (num) {
  switch (num) {
    case 1: {
      console.log("one");
      break;
    }
    case 2: {
      console.log("two");
      break;
    }
    default: {
      console.log("other");
      break;
    }
  }
};
match(1);
match(2);
match(99);
```

```js
const match_no_break = function (num) {
  switch (num) {
    case 1:
      console.log("one");
    case 2:
      console.log("two");
    default:
      console.log("other");
  }
};
match_no_break(1);
match_no_break(2);
match_no_break(99);
```

### For

```js
const arr = [1, 2, 3, 4];

for (let i = 0; i < arr.length; i++) {
  if (i % 2) {
    continue; // skip odd index
  }

  console.log(arr[i] * 2);
}
```

## FizzBuzz sample

```js
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

fizzBuzz(20);
```
