const match = function (num) {
	// TODO: break
	switch (num) {
		case 1: {
			console.log("one");
			break;
		}
		case 2: {
			console.log("two");
			break;
		}
		default: {
			console.log("other");
			break;
		}
	}
};

console.log("--- break ---");
match(1);
match(2);
match(99);

const match_no_break = function (num) {
	// TODO: break
	switch (num) {
		case 1:
			console.log("one");
		case 2:
			console.log("two");
		default:
			console.log("other");
	}
};

console.log("--- no break ---");
match_no_break(1);
match_no_break(2);
match_no_break(99);
