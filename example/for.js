const arr = [1, 2, 3, 4];

for (let i = 0; i < arr.length; i++) {
	if (i % 2) {
		continue; // skip odd index
	}

	console.log(arr[i] * 2);
}
