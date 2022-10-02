Array.prototype = {
	join: function (sep) {
		let s = "";

		// TODO: impl for statement
		const loop = function (i) {
			if (i > this.length) return;
			if (s !== "") {
				s = s + sep;
			}
			s = s + this[i];
			loop(i + 1);
		};

		loop(0);

		return s;
	},
};