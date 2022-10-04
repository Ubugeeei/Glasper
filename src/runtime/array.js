Array.prototype = {
	at: function (index) {
		if (index < 0) {
			index = index + this.length;
		}
		return this[index];
	},

	concat: function (v) {
		const c = [];
		for (let i = 0; i < this.length; i++) {
			c[i] = this[i];
		}
		for (let i = 0; i < v.length; i++) {
			c[i + this.length + 2] = v[i];
		}
		return c;
	},

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
	},
};
