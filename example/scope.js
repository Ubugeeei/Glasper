let v = 1;
let global = 100;
{
	let v = 2;
	console.log("child scope v:", v);
	console.log("global in child:", global);
}
console.log("parent scoped v:", v);
