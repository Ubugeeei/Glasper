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

	map: function (callback) {
		const m = [];
		for (let i = 0; i < this.length; i++) {
			m[i] = callback(this[i], i);
		}
		return m;
	},

	find: function (callback) {
		for (let i = 0; i < this.length; i++) {
			if (callback(this[i], i)) {
				return this[i];
			}
		}
		return undefined;
	}

	// forEach: function (callback) {
	// 	for (let i = 0; i < this.length; i++) {
	// 		callback(this[i], i);
	// 	}
	// },
};
