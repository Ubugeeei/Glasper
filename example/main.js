let num = 1;

if (num % 15 == 0) {
	console_log(true, true)
} else if (num % 5 == 0) {
	console_log(false, true)
} else if (num % 3 == 0) {
	console_log(true, false)
} else {
	console_log(num)
}


const isEven = function(num) {
	return num % 2 == 0;
}

console_log(isEven(1));
console_log(isEven(5));
console_log(isEven(6));
