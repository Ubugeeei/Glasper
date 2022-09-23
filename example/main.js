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
	console.log("null:", null); // (Object)
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
