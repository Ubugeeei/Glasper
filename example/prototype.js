Array.prototype = {
	nth: function (n) {
		return this[n]
	},
};

const arr = [1, 2, 3, 4, 5];
console.log("arr.nth(2):", arr.nth(2));
