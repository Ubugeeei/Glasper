/**
 * 
 * if statement branch
 * 
 */
const isEven = function(num) {
  if (num % 2) {
    return true;
  } else {
    return false;
  }
}
console_log("isEven(1):", isEven(1));
console_log("isEven(5):", isEven(5));
console_log("isEven(6):", isEven(6));

/**
 * 
 * recursive fizzBuzz
 * 
 */
const fizzBuzz = function(num) {
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
}
console_log("=== fizzBuzz(20) start ===");
fizzBuzz(20);
console_log("=== fizzBuzz(20) end ===");

/**
 * 
 * recursive factorial
 * 
 */
const factorial = function(num) {
  if (num == 0) {
    return 1;
  } else {
    return num * factorial(num - 1);
  }
}
console_log("factorial(5):", factorial(5));