const arr = [1, 2, 3, 4, 5];

console.log("arr[0]:", arr[0]);
console.log("arr[100]:", arr[100]);

console.log("arr.length:", arr.length);

const last = arr.at(-1);
console.log("last:", last);

const concat = arr.concat([6, 7, 8]);
console.log("concat[6]", concat[6]);

const every = arr.every(function (v) {
	return v < 10;
});
console.log("every:", every);

const joined = arr.join("/");
console.log("joined:", joined);

const mapped = arr.map(function (it) {
	return it * 2;
});
console.log("mapped[0]:", mapped[0]);

const found = arr.find(function (it) {
	return it % 2 === 0;
});
console.log("found:", found);
