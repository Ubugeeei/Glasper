const inner_f = function () {
	return this.value;
};
console.log("inner_f() -> ", inner_f());

const o = {
	value: 1,
	f: inner_f,
};
console.log("o.f() -> ", o.f());

const oo = {
	value: 2,
	f: inner_f,
};
console.log("oo.f() -> ", oo.f());
console.log("this.value", this.value);


