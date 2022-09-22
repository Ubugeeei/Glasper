install:
	cargo build --release
	cp target/release/gls /usr/local/bin/