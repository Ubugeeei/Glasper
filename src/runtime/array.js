Array.prototype = {
	join: function (sep) {
		let s = "";

		for (let i = 0; i < this.length; i++) {
			if (i > 0) {
				s = s + sep;
			}
			s = s + this[i];
		}

		return s;
	},

	// TODO:
	// map: function (callback) {
	// 	const a = [];

	// 	for (let i = 0; i < this.length; i++) {
	// 		a[i] = callback(this[i], i);
	// 	}

	// 	return a;
	// }
};
