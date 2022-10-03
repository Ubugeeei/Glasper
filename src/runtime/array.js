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
};
