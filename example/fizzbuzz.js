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
