const fizzBuzz = function(num) {
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

fizzBuzz(20);


const isEven = function(num) {
  if (num % 2) {
    return true;
  } else {
    return false;
  }
}
console_log(isEven(1));
console_log(isEven(5));
console_log(isEven(6));

const factorial = function(num) {
  if (num == 0) {
    return 1;
  } else {
    return num * factorial(num - 1);
  }
}
console_log(factorial(5));